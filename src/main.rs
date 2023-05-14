#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_getters;

pub mod logger;
pub mod cli;
pub mod messages;
pub mod files;
pub mod serde;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    logger::initialize_logger(*cli.debug()).expect("failed to initialize logger");

    files::load_config().unwrap();

    cli.command().execute()
}
