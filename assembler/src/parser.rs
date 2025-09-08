#![allow(non_snake_case)]
#![allow(unused)]

use crate::lexer::{Token, ZsmTokens};
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};
use std::thread;
use stdlib::opcode::*;

fn ConvertLexerToBytesCvt(tokens: &ZsmTokens) -> Result<Vec<u8>> {
    let mut sections: [ZsmTokens; 3] = [ZsmTokens::new(), ZsmTokens::new(), ZsmTokens::new()]; // global, data, code
    let mut bytecode: Vec<u8> = Vec::new();
    bytecode.reserve(100);

    split_sections(tokens, &mut sections)?;

    // convert tokens to bytecode
    for j in 0..3 {
        for i in 0..sections[j].tokens.len() {
            match &sections[j].tokens[i] {
                Token::Key(ky) => bytecode.push(*ky as u8),
                Token::Value(vl) => {
                    let str_bytes = vl.as_bytes();
                    bytecode.push(KEY_STR_BEGIN);
                    bytecode.push(str_bytes.len() as u8);
                    for i in 0..str_bytes.len() {
                        bytecode.push(str_bytes[i]);
                    }
                }
            }
        }
    }

    Ok(bytecode)
}

pub fn multi_cltbts(tokens: Arc<Mutex<Vec<ZsmTokens>>>) -> Result<Vec<Vec<u8>>> {
    let mut bytecodes: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::new()));
    let tok_len = tokens.lock().unwrap().len();
    let part1 = tok_len / 2;

    thread::scope(|s| {
        let tok1 = Arc::clone(&tokens);
        let tok2 = Arc::clone(&tokens);
        let btc = Arc::clone(&bytecodes);
        let btc1 = Arc::clone(&bytecodes);

        s.spawn(move || {
            for i in 0..part1 {
                let tok = &tok1.lock().unwrap()[i];
                btc.lock()
                    .unwrap()
                    .push(ConvertLexerToBytesCvt(tok).unwrap());
            }
        });
        s.spawn(move || {
            for i in part1..tok_len {
                let tok = &tok2.lock().unwrap()[i];
                btc1.lock()
                    .unwrap()
                    .push(ConvertLexerToBytesCvt(tok).unwrap());
            }
        });
    });

    let btcodes = bytecodes.lock().unwrap().clone();
    Ok(btcodes)
}

fn split_sections(tokens: &ZsmTokens, sections: &mut [ZsmTokens; 3]) -> Result<()> {
    let mut index: usize = 0;
    let mut current_section: u8 = 0;
    let mut InSectionKey: bool = false;

    while index < tokens.tokens.len() {
        match &tokens.tokens[index] {
            Token::Key(key) => {
                if InSectionKey {
                    return Err(Error::new(
                        ErrorKind::Unsupported,
                        "keyword as section name not supported",
                    ));
                }
                if *key == Keys::Section {
                    InSectionKey = true;
                } else {
                    sections[current_section as usize]
                        .tokens
                        .push(Token::Key(*key));
                    sections[current_section as usize]
                        .split_word
                        .push(tokens.split_word[index].clone());
                }
            }
            Token::Value(val) => {
                if InSectionKey {
                    match val.as_str() {
                        "data" => current_section = 1,
                        "readable" => current_section = 1,
                        "info" => current_section = 1,
                        "code" => current_section = 2,
                        "text" => current_section = 2,
                        "end" => current_section = 0,
                        _ => {
                            return Err(Error::new(
                                ErrorKind::Unsupported,
                                "section type not supported",
                            ))
                        }
                    }
                    InSectionKey = false;
                } else {
                    sections[current_section as usize]
                        .tokens
                        .push(Token::Value(val.to_string()));
                    sections[current_section as usize]
                        .split_word
                        .push(tokens.split_word[index].clone());
                }
            }
        }

        index += 1;
    }
    Ok(())
}
