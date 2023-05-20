use std::process::Command;

use anyhow::Result;

use crate::fs::dedicated_server_dir;

pub fn start_server() -> Result<()> {
    if cfg!(target_os = "windows") {
        start_server_windows()
    } else if cfg!(target_os = "linux") {
        start_server_linux()
    } else {
        panic!("must run server on linux or windows")
    }
}

fn start_server_windows() -> Result<()> {
    info!("Starting Windows server using StartServer64.bat");

    let dir = dedicated_server_dir()?;

    debug!("Changing working directory to: [{dir}]");
    std::env::set_current_dir(&dir)?;

    let _ = Command::new("cmd")
        .args(&["/C", "StartServer64.bat"])
        .spawn()
        .expect("failed server start");

    Ok(())
}

fn start_server_linux() -> Result<()> {
    info!("Starting Linux server using start_server.sh");

    let dir = dedicated_server_dir()?;

    debug!("Changing working directory to: [{dir}]");
    std::env::set_current_dir(&dir)?;

    let _ = Command::new("\\bin\\bash")
        .args(&["start_server.sh"])
        .spawn()
        .expect("failed server start");

    Ok(())
}
