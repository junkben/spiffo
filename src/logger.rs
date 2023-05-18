use log::{LevelFilter, SetLoggerError};

struct SpiffoLogger;

impl log::Log for SpiffoLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        let msg = if record.level() >= log::Level::Debug {
            log_msg(record)
        } else {
            log_msg(record)
        };
        println!("{msg}")
    }

    fn flush(&self) {}
}

//fn debug_msg(record: &log::Record) -> String {
//    use chrono::Local;
//    format!(
//        "[{}][SPIFFO][{}] {}",
//        Local::now().format("%Y-%m-%d %H:%M:%S"),
//        record.level(),
//        record.args(),
//    )
//}

fn log_msg(record: &log::Record) -> String {
    format!(
        "{} {}",
        format!(
            "{:width$}",
            format!("[SPIFFO][{}]", record.level()),
            width = 15,
        ),
        record.args()
    )
}

static LOGGER: SpiffoLogger = SpiffoLogger;

pub fn initialize_logger(debug: bool) -> Result<(), SetLoggerError> {
    let log_level = match debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    log::set_logger(&LOGGER)?;
    log::set_max_level(log_level);

    debug!("logger initialized at {} level", log_level);
    Ok(())
}
