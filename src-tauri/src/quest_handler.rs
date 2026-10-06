// read the testing json
// turn into json
// make worker exe
// launch worker exe
use struct_iterable::Iterable;

use std::{
    io::{self},
    path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Executable {
    pub os: String,
    pub name: String,
    pub is_launcher: bool,
}

#[derive(Serialize, Deserialize, Debug, Iterable)]
pub struct QuestData {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub summary: String,

    pub r#type: i8,

    pub is_monetized: bool,
    pub is_verified: bool,
    pub is_discoverable: bool,

    pub cover_image: String,

    pub executables: Vec<Executable>,
}

pub type Quests = Vec<QuestData>;

pub fn parse_data(quest_data: String) -> io::Result<QuestData> {
    // let f = File::open("arknights.json")?; // temporary
    let file_json = serde_json::from_str(&quest_data)?;

    Ok(file_json)
}

pub fn get_valid_exe(quest_data: &QuestData) -> Option<&Executable> {
    quest_data
        .executables
        .iter()
        .find(|x| x.os == "win32" && x.is_launcher == false)
}

pub fn get_exe_path(quest_data: &QuestData) -> path::PathBuf {
    let executable = get_valid_exe(quest_data).unwrap();

    let mut exe_path = path::PathBuf::from("executables");
    exe_path.push(executable.name.clone());

    exe_path
}
