#![allow(unused)]

// global type specifier
pub const KEY_STR_BEGIN: u8 = 98;

// opcodes
#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Keys {
    Section = 1,
    Func = 2,
    Mov = 3,
    // End,
    Alloc = 4,
    Dealloc = 5,
    Lptr = 6,
    Sptr = 7,
    Call = 8,
    Jmp = 9,
    Jne = 10,
    Je = 11,
    Jg = 12,
    Jge = 13,
    Jng = 14,
    Jnge = 15,
    Cmp = 16,
    Ptr = 17,
    UPtr = 18,
}

// data types
const TYPE_CHAR: u8 = 1;
const TYPE_INT: u8 = 2;
const TYPE_FLOAT: u8 = 3;
const TYPE_PTR: u8 = 4;
