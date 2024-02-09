mod consts;

pub use consts::*;
use serde::{Deserialize, Serialize};

use crate::fs::{ReadFromFile, WriteToFile};

// pub fn start_server_default() -> Result<()> {
//     info!("Starting Windows server with default java args");
//
//     let dir = dedicated_server_dir()?;
//
//     debug!("Changing working directory to: [{dir}]");
//     std::env::set_current_dir(&dir)?;
//
//     let java_args = JavaArgs::default();
//     debug!("Setting PZ_CLASSPATH={}", java_args.class_path);
//     std::env::set_var("PZ_CLASSPATH", java_args.class_path);
//
//     let program = "jre64\\bin\\java.exe";
//     let cmd_args = [
//         &java_args.vm_args,
//         "-cp",
//         &java_args.class_path,
//         &java_args.main_class,
//         "-statistic 0 %1 %2",
//     ];
//
//     debug!("Starting server with command: {program} {}", cmd_args.join(" "));
//     let _ = Command::new(program)
//         .args(cmd_args)
//         .env("PZ_CLASSPATH", &java_args.class_path)
//         .spawn()
//         .context("failed to start server")?;
//
//     Ok(())
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaArgs {
    main_class: String,
    class_path: String,
    vm_args:    String,
    file_path:  String
}

impl WriteToFile for JavaArgs {}

impl ReadFromFile for JavaArgs {}
