use core::time;
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    ptr::null,
    sync::{Arc, Mutex},
    thread::sleep,
};

use futures_util::{sink::Buffer, SinkExt, StreamExt};
use reqwest::Response;
use serde::Deserialize;
use serde_json::{from_str, json, Deserializer};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Utf8Bytes},
};

use crate::{
    quest_handler::{self, QuestData, Quests},
    worker_handler::{self, WorkerType},
};

#[derive(Deserialize)]
struct DiscordWebSocket {
    description: String,
    devtoolsFrontendUrl: String,
    id: String,
    title: String,
    r#type: String,
    url: String,
    webSocketDebuggerUrl: String,
}

#[derive(Deserialize, Debug)]
struct DevConsoleResult {
    r#type: String,
    value: String,
}

#[derive(Deserialize, Debug)]
struct DevConsoleResultParent {
    result: DevConsoleResult,
}

#[derive(Deserialize, Debug)]
struct DevConsoleResponse {
    id: i8,
    result: DevConsoleResultParent,
}

pub type ArcWorkers = Arc<Mutex<WorkerType>>;

async fn get_websocket_url() -> String {
    let mut websocket_url = "".to_string();

    while websocket_url == "" {
        let body = reqwest::get("http://localhost:9222/json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let parsed_response: Vec<DiscordWebSocket> = serde_json::from_str(&body).unwrap();
        for x in parsed_response {
            if let Some(_) = x.url.find("discord.com/channels") {
                websocket_url = x.webSocketDebuggerUrl;
            }
        }

        sleep(time::Duration::from_millis(100));
    }

    websocket_url
}

fn emulate_game(quest_info: &QuestData, workers: ArcWorkers) -> io::Result<()> {
    let executable_path = quest_handler::get_exe_path(quest_info);
    worker_handler::create_worker_executable(&executable_path);

    let worker =
        worker_handler::launch_worker_executable(quest_info.name.clone(), &executable_path);
    if let Ok(mut locked_workers) = workers.lock() {
        locked_workers.push(worker);
    }

    Ok(())
}

fn emulate_games(quests: Quests, workers: ArcWorkers) -> io::Result<()> {
    for x in quests.iter() {
        emulate_game(x, Arc::clone(&workers)).unwrap();
    }

    Ok(())
}

fn process_websocket_data(message: &Utf8Bytes) -> io::Result<Quests> {
    let response: DevConsoleResponse = serde_json::from_str(message.as_str())?;
    let quest_info: Quests = serde_json::from_str(&response.result.result.value)?;

    println!("Quest Info {:?}", quest_info);

    Ok(quest_info)
}

async fn connect_websoccket(websocket_url: &String, workers: ArcWorkers) -> io::Result<()> {
    let (ws_stream, response) = connect_async(websocket_url).await.unwrap();
    let (mut write, mut read) = ws_stream.split();

    let script_path = File::open("scriptToExecute.js")?;
    let js_reader = BufReader::new(script_path);

    let message_to_send = json!({
        "id": 1,
        "method": "Runtime.evaluate",
        "params": {
            "expression": io::read_to_string(js_reader)?,
            "returnByValue": true,
            "awaitPromise": true
        }
    });

    let message_as_json = serde_json::to_string(&message_to_send).unwrap();

    let rawr = Utf8Bytes::from(&message_as_json);
    write.send(Message::Text(rawr)).await.unwrap();

    tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                let worker_for_ws = workers.clone();
                let process_result = process_websocket_data(&text);

                match process_result {
                    // Ok(quest_info) => emulate_game(quest_info, worker_for_ws).unwrap(),
                    Ok(quest_info) => emulate_games(quest_info, worker_for_ws).unwrap(),
                    Err(e) => {
                        println!("err {e}\n{}", text.as_str());
                    }
                }
            }
        }
    });

    println!("Connected: {:?}", response.status());

    Ok(())
}

pub async fn init(workers: ArcWorkers) {
    let websocket_url = get_websocket_url().await;
    let websocket = connect_websoccket(&websocket_url, workers).await;

    match websocket {
        Ok(()) => (),
        Err(error) => println!("Do you have scriptToExecute.js?\n{}", error),
    };

    println!("{:?}", websocket_url)
}
