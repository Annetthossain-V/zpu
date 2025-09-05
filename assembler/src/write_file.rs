#![allow(unused)]

use std::fs::File;
use std::io::{Result, Write};
use std::sync::Arc;

pub const ZPU_VERSION: u8 = 1;
pub const ZSM_VERSION: u8 = 1;
pub const ARCH: u8 = 32;

pub fn write_file_header(file_name: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(file_name).unwrap();
    let extension = std::path::Path::new(file_name).extension().unwrap();
    if (extension != "zo") {
        panic!("Invalid File Extension {}", file_name);
    }

    let file_header: &[u8] = &[
        55,
        92,
        210,
        b'Z',
        b'P',
        b'U',
        b' ',
        b'O',
        b'B',
        b'J',
        b' ',
        b'A',
        b'H',
        b' ',
        ZPU_VERSION,
        ZSM_VERSION,
        ARCH,
    ];
    file.write_all(file_header)?;
    file.write_all(data)?;
    Ok(())
}

pub fn multi_file_write(data: Vec<Vec<u8>>, names: Arc<Vec<String>>) -> Result<()> {
    Ok(())
}
