mod flag;

use flag::Flags;
use std::io::Result;

fn main() -> Result<()> {
    let mut flags = Flags::new();
    flags.parse()?;
    flags.info();

    Ok(())
}
