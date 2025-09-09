#![allow(unused_imports)]

mod flag;

use flag::Flags;
use log::{debug, error, info, trace, warn};
use simple_logger::SimpleLogger;
use std::io::Result;
use stdlib::data;

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut flags = Flags::new();
    flags.parse()?;
    flags.info();

    debug!("initializing global variables");
    data::register_init();

    Ok(())
}
