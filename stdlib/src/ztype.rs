#![allow(unused)]
use std::io::Result;

enum ZTypeList {
    ZStr, // utf-16 
    ZPtr,
    Float,
    Char, // basic c string
    Int,
}

trait ZType<T> {
   fn new() -> Self;
   fn clone(&self) -> Self;
   fn data(&self) -> T;
   fn push(&mut self);
   fn pop(&mut self);
   fn nth(&self, i: usize) -> T;
   fn remove_nth(&mut self, i: usize) -> Result<()>;
   fn insert(&mut self, i: usize) -> Result<()>;
}

// impl to_string for storing strings in this
// basically c-strings
struct ZPtr<T> {
    data: Vec<T>,
    data_type: ZTypeList,
    len: u32,
    cap: u32,
    mutable: bool,
}

struct ZStr {
    data: ZPtr<u16>,
    mutable: bool,
    rstr: Box<str>,
}

impl<T> ZType<T> for ZPtr<T> {

}

impl<T> ZType<T> for ZStr {

}



