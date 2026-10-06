use std::{
    path::Path,
    process::Child,
    sync::{Arc, Mutex},
};

use serde::Serialize;
use tauri::{App, AppHandle, Emitter, RunEvent};

use crate::{client_handler::ArcWorkers, worker_handler::clean_workers};

mod client_handler;
mod discord_handler;
mod quest_handler;
mod worker_handler;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn is_worker() -> bool {
    std::env::args().any(|a| a == "--worker")
}

#[tauri::command]
fn get_app_name() -> String {
    // let current_exe = std::env::current_exe().unwrap();
    // let name = Path::new(&current_exe)
    //     .file_name()
    //     .unwrap()
    //     .to_str()
    //     .unwrap();

    // String::from(name)

    std::env::args().nth(2).unwrap()
}

#[derive(Clone, Serialize)]
struct TestEvent {
    test: i16,
}

fn report_to_client(app: AppHandle) {
    app.emit::<TestEvent>("test-event", TestEvent { test: 12 })
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let is_worker_flag = is_worker();

    let workers = Arc::new(Mutex::new(Vec::new()));

    let workers_for_tauri = Arc::clone(&workers);

    if !is_worker_flag {
        discord_handler::launch_discord();

        tauri::async_runtime::spawn(async {
            client_handler::init(workers).await;
        });
    }

    println!("Is worker : {}", is_worker_flag);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, is_worker, get_app_name])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(move |app_handle, event| match event {
            RunEvent::ExitRequested { .. } => {
                if let Ok(mut locked_workers) = workers_for_tauri.lock() {
                    clean_workers(&mut locked_workers);
                };
            }

            RunEvent::Ready => {
                report_to_client(app_handle.clone());
            }

            _ => {}
        });
}
