mod consts;

use anyhow::{Result, Context};
pub use consts::*;
use serde::{Deserialize, Serialize};
use std::{process::Command, io::Write};

use crate::fs::{java_executable_path, java_binary_dir, java_executable_filename, dedicated_server_dir};

pub fn start_server_default() -> Result<()> {
    info!("Starting Windows server with default java args");

    let dir = dedicated_server_dir()?;

    debug!("Changing working directory to: [{dir}]");
    std::env::set_current_dir(&dir)?;

    let java_args = JavaArgs::default();
    debug!("Setting PZ_CLASSPATH={}", &java_args.class_path_string());
    std::env::set_var("PZ_CLASSPATH", &java_args.class_path_string());

    let program = "jre64\\bin\\java.exe";
    let cmd_args = [
        &java_args.vm_args_string(),
        "-cp",
        &java_args.class_path_string(),
        &java_args.main_class,
        "-statistic 0 %1 %2",
    ];
    
    debug!("Starting server with command: {program} {}", cmd_args.join(" "));
    let _ = Command::new(program)
        .args(cmd_args)
        .env("PZ_CLASSPATH", &java_args.class_path_string())
        .spawn()
        .context("failed to start server")?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaArgs {
    main_class: String,
    class_path: Vec<String>,
    vm_args: Vec<String>,
}

impl Default for JavaArgs {
    fn default() -> JavaArgs {
        JavaArgs {
            main_class: MAIN_CLASS.to_string(),
            class_path: Vec::from(CLASS_PATH_64.map(|a| a.to_string())),
            vm_args: Vec::from(VM_ARGS_64.map(|a| a.to_string())),
        }
    }
}

impl JavaArgs {
    fn class_path_string(&self) -> String {
        self.class_path.join(";")
    }

    fn vm_args_string(&self) -> String {
        self.vm_args.join(" ")
    }
}
