#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_getters;

pub mod cli;
pub mod fs;
pub mod logger;
pub mod serde;
pub mod settings;

use clap::Parser;
use cli::SpiffoCLI;

fn main() {
    let cli = SpiffoCLI::parse();

    logger::initialize_logger(*cli.debug()).expect("failed to initialize logger");

    if let Err(err) = cli.command().execute() {
        error!("spiffo failed: {err}");
    }
}
