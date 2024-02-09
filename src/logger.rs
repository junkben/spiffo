use log::*;

struct SpiffoLogger;

impl Log for SpiffoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool { metadata.level() <= Level::Debug }

    fn log(&self, record: &Record) {
        let msg = if record.level() >= Level::Debug {
            log_msg(record)
        } else {
            log_msg(record)
        };
        println!("{msg}")
    }

    fn flush(&self) {}
}

// fn debug_msg(record: &Record) -> String {
//    format!(
//        "[{}][SPIFFO][{}] {}",
//        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
//        record.level(),
//        record.args(),
//    )
//}

fn log_msg(record: &Record) -> String {
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
        false => LevelFilter::Info
    };

    set_logger(&LOGGER)?;
    set_max_level(log_level);

    debug!("logger initialized at {} level", log_level);
    Ok(())
}
