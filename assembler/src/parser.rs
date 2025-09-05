#![allow(non_snake_case)]
#![allow(unused)]

use crate::lexer::ZsmTokens;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::thread;

fn ConvertLexerToBytesCvt(tokens: &ZsmTokens) -> Result<Vec<u8>> {
    let mut sections: [ZsmTokens; 3] = [ZsmTokens::new(), ZsmTokens::new(), ZsmTokens::new()]; // global, data, code
    split_sections(tokens, &mut sections);

    Ok(Vec::new())
}

pub fn multi_cltbts(tokens: Arc<Mutex<Vec<ZsmTokens>>>) -> Result<Vec<Vec<u8>>> {
    Ok(Vec::new())
}

fn split_sections(tokens: &ZsmTokens, sections: &mut [ZsmTokens; 3]) -> Result<()> {
    let mut index: usize = 0;
    let mut current_section: u8 = 0;

    while index < tokens.tokens.len() {
        println!("word {}", tokens.split_word[index]);
        index += 1;
    }
    Ok(())
}
