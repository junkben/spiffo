#[macro_use] extern crate log;
#[macro_use] extern crate derive_getters;

pub mod cli;
pub mod fs;
pub mod java;
pub mod logger;
pub mod serde;
pub mod server;

use clap::Parser;

fn main() {
    let cli = cli::SpiffoCLI::parse();

    logger::initialize_logger(*cli.debug()).expect("failed to initialize logger");

    match cli.command().execute() {
        Ok(()) => info!("spiffo executed successfully"),
        Err(err) => error!("spiffo failed: {err}")
    }
}
