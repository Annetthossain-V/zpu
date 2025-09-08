#![allow(unused)]

use std::fs::File;
use std::io::{Error, ErrorKind, Result, Write};
use std::sync::Arc;
use std::thread;

pub const ZPU_VERSION: u8 = 1;
pub const ZSM_VERSION: u8 = 1;
pub const ARCH: u8 = 32;

pub fn write_file_single(file_name: &str, data: &[u8]) -> Result<()> {
    let mut file = File::create(file_name).unwrap();
    let extension = std::path::Path::new(file_name).extension().unwrap();
    if (extension != "zob") {
        return Err(Error::new(ErrorKind::InvalidFilename, "file type not .zob"));
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

pub fn multi_file_write(data: &Vec<Vec<u8>>, names: Arc<Vec<String>>) -> Result<()> {
    let data_shared = Arc::new(data);
    let item_len = data.len();
    let p1 = item_len / 2;

    thread::scope(|s| {
        let data_shared1 = Arc::clone(&data_shared);
        let data_shared2 = Arc::clone(&data_shared);
        let names1 = Arc::clone(&names);
        let names2 = Arc::clone(&names);

        // --- first thread ---
        s.spawn(move || {
            for i in 0..p1 {
                let mut str = names1[i].clone();
                str = str.replace("zsm", "zob");
                write_file_single(&str, &data_shared1[i]).unwrap();
            }
        });
        s.spawn(move || {
            for i in p1..item_len {
                let mut str = names2[i].clone();
                str = str.replace("zsm", "zob");
                write_file_single(&str, &data_shared2[i]).unwrap();
            }
        });
    });
    Ok(())
}
