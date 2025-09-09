#![allow(non_upper_case_globals)]
#![allow(unused)]

use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::env::var;
use std::sync::Mutex;

pub const REG_MODE_FLOAT: u8 = 4;
pub const REG_MODE_INTEGER: u8 = 6;
pub const REG_MODE_STRING: u8 = 34;

pub struct Reg {
    float: f32,
    integer: i32,
    string: String,
    mode: u8,
}

lazy_static! {
    pub static ref GlobalVariable: Mutex<HashMap<String, Reg>> = Mutex::new(HashMap::new());
}

pub fn register_init() {
    let register_count: u16;
    match var("ZPU_REG_COUNT") {
        Ok(val) => {
            register_count = val.parse::<u16>().unwrap();
            debug!("Register Count: {}", register_count);
        }
        _ => register_count = 16,
    }

    for i in 0..register_count {
        let mut reg_name = String::from("r"); // prefix
        let reg: Reg = Reg {
            float: 0.0,
            integer: 0,
            string: String::new(),
            mode: REG_MODE_INTEGER,
        };
        reg_name.push_str(i.to_string().as_str());
        GlobalVariable.lock().unwrap().insert(reg_name, reg);
    }
}
