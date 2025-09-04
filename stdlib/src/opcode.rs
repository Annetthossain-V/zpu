#![allow(unused)]

const OPCODE_MOV: u16 =     1;
const OPCODE_LPTR: u16 =    2;
const OPCODE_SPTR: u16 =    3;
const OPCODE_GPTR: u16 =    4;
const OPCODE_ALLOC: u16 =   5;

const TYPE_CHAR: u16 =      1;
const TYPE_INT: u16 =       2;
const TYPE_FLOAT: u16 =     3;
const TYPE_PTR: u16 =       4;

const BYTE_TYPE_U8: u8 =    1;
const BYTE_TYPE_U16: u8 =   2;
const BYTE_TYPE_U32: u8 =   3;
const BYTE_TYPE_U64: u8 =   4;
const BYTE_TYPE_F32: u8 =   5;
const BYTE_TYPE_F16: u8 =   6;
