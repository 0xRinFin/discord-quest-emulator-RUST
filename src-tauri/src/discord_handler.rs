use std::{
    env,
    ffi::OsStr,
    os::windows::process::CommandExt,
    path,
    process::{Command, Stdio},
};

use sysinfo::{ProcessRefreshKind, RefreshKind};

pub fn kill_running_disccord() {
    let mut system = sysinfo::System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::nothing()),
    );

    system.refresh_all();

    system
        .processes_by_name(&OsStr::new("Discord.exe"))
        .for_each(|x| {
            x.kill();
        });
}

pub fn launch_discord() {
    let path_pattern = format!(
        "{}\\Discord\\app-*\\Discord.exe",
        std::env::var("LOCALAPPDATA").unwrap()
    );

    let discord_path = glob::glob(&path_pattern).unwrap().flatten().next().unwrap();
    println!("{:?}", discord_path);

    // kill_running_disccord();
    let _ = Command::new(discord_path)
        .args(["--remote-debugging-port=9222", "--remote-allow-origins=*"])
        .stdin(Stdio::null()) // Disconnect input
        .stdout(Stdio::null()) // Disconnect output
        .stderr(Stdio::null()) // Disconnect errors
        .creation_flags(0x00000008)
        .spawn()
        .unwrap();
}
