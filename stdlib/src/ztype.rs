#![allow(unused)]
use std::io::{Error, ErrorKind, Result};
use std::ops::{Index, IndexMut};

// impl to_string for storing strings in this
// basically c-strings
struct ZPtr<T: Clone> {
    data: Vec<T>,
    pub len: u32,
    pub cap: u32,
    pub mutable: bool,
}

struct Zstr {
    data: ZPtr<u16>,
}

impl<T: Clone> ZPtr<T> {
    pub fn new(_cap: u32, _mut: bool) -> Self {
        ZPtr {
            data: Vec::new(),
            len: 0,
            cap: _cap,
            mutable: _mut,
        }
    }
    pub fn clone(&self) -> Self {
        ZPtr {
            data: self.data.clone(),
            len: self.len,
            cap: self.cap,
            mutable: self.mutable,
        }
    }
    pub fn push(&mut self, item: T) -> Result<()> {
        self.len += 1;
        self.data.push(item);
        if self.len > self.cap {
            return Err(Error::new(
                ErrorKind::StorageFull,
                "length larger than capacity",
            ));
        }
        Ok(())
    }
}

impl Zstr {
    pub fn new(_mut: bool) -> Self {
        Zstr {
            data: ZPtr::<u16>::new(2000000, _mut),
        }
    }
}
