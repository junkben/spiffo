#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_getters;

pub mod cli;
pub mod files;
pub mod logger;
pub mod serde;
pub mod settings;

use clap::Parser;
use cli::SpiffoCLI;

fn main() {
    let cli = SpiffoCLI::parse();

    logger::initialize_logger(*cli.debug()).expect("failed to initialize logger");

    // files::load_config().unwrap();

    cli.command().execute()
}
