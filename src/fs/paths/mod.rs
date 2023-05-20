use anyhow::Result;
use std::env;

pub fn home() -> Result<String> {
    let home = env::var("HOME")?;
    Ok(home.replace("\\", "/"))
}

macro_rules! home_relative_dirs {
    ($($const_name:ident, $fn_name:ident, $const_value:expr);*) => {
        $(
            const $const_name: &'static str = $const_value;

            pub fn $fn_name() -> Result<String> {
                let home = home()?;
                let dir = env::var(stringify!($const_name)).unwrap_or($const_name.to_string());
                let path = [home, dir].join("/");
                debug!("{}: {}", stringify!($fn_name), path);
                Ok(path)
            }
        )*
    }
}

pub fn dedicated_server_dir() -> Result<String> {
    let path = "C:/Program Files (x86)/Steam/steamapps/common/Project Zomboid Dedicated Server".to_owned();
    debug!("dedicated_server_dir: {path}");
    Ok(path)
}

pub fn java_binary_dir() -> Result<String> {
    let path = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Project Zomboid Dedicated Server\\jre64\\bin".to_owned();
    debug!("java_binary_dir: {path}");
    Ok(path)
}

home_relative_dirs!(
    //DEDICATED_SERVER_DIR, dedicated_server_dir, ".steam/steam/steamapps/common/'Project Zomboid Dedicated Server'";
    //JAVA_BINARY_DIR, java_binary_dir, ".steam/steam/steamapps/common/'Project Zomboid Dedicated Server'/jre64/bin";
    SERVER_DIR, server_dir, "Zomboid/Server";
    SAVES_DIR, saves_dir, "Zomboid/Saves/Multiplayer"
);

macro_rules! zomboid_server_filenames {
    ($($const_name:ident, $fn_name:ident, $const_value:expr);*) => {
        $(
            const $const_name: &'static str = $const_value;

            pub fn $fn_name() -> Result<String> {
                let dir = env::var(stringify!($const_name)).unwrap_or($const_name.to_string());
                debug!("{}: {}", stringify!($fn_name), dir);
                Ok(dir)
            }
        )*
    }
}

zomboid_server_filenames!(
    ZOMBOID_CONFIG_FILENAME, zomboid_config_filename, "servertest.ini";
    ZOMBOID_SANDBOX_VARS_FILENAME, zomboid_sandbox_vars_filename, "servertest_SandboxVars.lua";
    ZOMBOID_SPAWN_REGIONS_FILENAME, zomboid_spawn_regions_filename, "servertest_spawnregions.lua";
    JAVA_CONFIG_FILENAME, java_config_filename, "ProjectZomboid64.json";
    JAVA_EXECUTABLE_FILENAME, java_executable_filename, "java.exe"
);

pub fn zomboid_config_path() -> Result<String> {
    let dir = server_dir()?;
    let file = zomboid_config_filename()?;
    let path = [dir, file].join("/");
    debug!("zomboid_config_path: {}", path);
    Ok(path)
}

pub fn zomboid_sandbox_vars_path() -> Result<String> {
    let dir = server_dir()?;
    let file = zomboid_spawn_regions_filename()?;
    let path = [dir, file].join("/");
    debug!("zomboid_sandbox_vars_path: {}", path);
    Ok(path)
}

pub fn zomboid_spawn_regions_path() -> Result<String> {
    let dir = server_dir()?;
    let file = zomboid_spawn_regions_filename()?;
    let path = [dir, file].join("/");
    debug!("zomboid_spawn_regions_path: {}", path);
    Ok(path)
}

pub fn java_config_path() -> Result<String> {
    let dir = dedicated_server_dir()?;
    let file = java_config_filename()?;
    let path = [dir, file].join("/");
    debug!("java_config_path: {}", path);
    Ok(path)
}

pub fn java_executable_path() -> Result<String> {
    let dir = java_binary_dir()?;
    let file = java_executable_filename()?;
    let path = [dir, file].join("/");
    debug!("java_executable_path: {path}");
    Ok(path)
}
