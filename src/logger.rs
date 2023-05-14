use log::{SetLoggerError, LevelFilter};

struct SpiffoLogger;

impl log::Log for SpiffoLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        println!("{}", compose_log_msg(record))
    }

    fn flush(&self) {}
}

fn compose_log_msg(record: &log::Record) -> String {
    // Makes the log prefix the same length no matter the log level
    let prefix = format!(
        "{:width$}",
        format!("[SPIFFO][{}]", record.level()),
        width = 16
    );

    // Put prefix and msg contents together
    let msg = format!("{} | {}", prefix, record.args());

    // // newspace every 100 characters
    // let msg = raw.chars()
    //     .enumerate()
    //     .flat_map(|(i, c)| {
    //         if i != 0 && i % 100 == 0 {
    //             Some('\n')
    //         } else {
    //             None
    //         }
    //         .into_iter()
    //         .chain(std::iter::once(c))
    //     })
    //     .collect::<String>();

    msg
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