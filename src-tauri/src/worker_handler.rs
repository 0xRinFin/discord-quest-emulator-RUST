use std::{
    env,
    fs::{self, create_dir_all},
    path,
    process::{Child, Command},
};

pub type WorkerType = Vec<Child>;

pub fn create_worker_path(executable_path: &path::PathBuf) {
    create_dir_all(executable_path.parent().unwrap()).unwrap();
}

pub fn create_worker_executable(executable_path: &path::PathBuf) {
    create_worker_path(&executable_path);

    let current_binary = env::current_exe().unwrap();
    fs::copy(current_binary, executable_path).unwrap();
}

pub fn launch_worker_executable(quest_info: String, executable_path: &path::PathBuf) -> Child {
    Command::new(executable_path)
        .arg("--worker")
        .arg(quest_info)
        .spawn()
        .unwrap()
}

pub fn clean_workers(workers: &mut Vec<Child>) {
    for worker in workers.iter_mut() {
        worker.kill().unwrap();
    }
}
