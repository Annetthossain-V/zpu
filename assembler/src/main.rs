
use std::io::Result;
use std::sync::{Mutex, Arc};
use std::thread;
use std::io::Read;


mod flag;
mod lexer;
#[allow(unused)]
use flag::{ 
    Flags,
    Options
};

use lexer::ZsmTokens;

fn main() -> Result<()> {
    let mut flags: Flags = Flags::new();
    flags.parse()?;
    flags.info();

    let files_data: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::new()));
    let files = Arc::new(flags.files);
    let lexed_tokens: Arc<Mutex<Vec<ZsmTokens>>> = Arc::new(Mutex::new(Vec::new()));

    multi_file_reader(files.len(), files.clone(), files_data.clone())?;
    multi_lexer(lexed_tokens.clone(), files_data.clone(), files.len())?; 

    
    Ok(())
}

fn single_file_reader(file_name: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(file_name)?;
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn multi_file_reader(files_count: usize, files: Arc<Vec<String>>, files_data: Arc<Mutex<Vec<Vec<u8>>>>) -> Result<()> {
    if files_count == 1 {
        let buf: Vec<u8> = single_file_reader(&files[0])?;
        files_data.lock().unwrap().push(buf);
        return Ok(());
    }

    let first_half_end = files_count / 2;

    thread::scope(|s| {
        let files_clone = Arc::clone(&files);
        let files_data_clone = Arc::clone(&files_data);

        // --- First Thread ---
        s.spawn(move || {
            for i in 0..first_half_end {
                let buffer = single_file_reader(&files_clone[i]);
                match buffer {
                    Ok(val) => files_data_clone.lock().unwrap().push(val),
                    Err(e) => eprintln!("Error Reading File {}, Error {}", &files_clone[i], e),
                }
            }
        });

        // --- Second Thread ---
        let files_clone2 = Arc::clone(&files);
        let files_data_clone2 = Arc::clone(&files_data);

        s.spawn(move || {
            for i in first_half_end..files_count {
                let buffer = single_file_reader(&files_clone2[i]);
                match buffer {
                    Ok(val) => files_data_clone2.lock().unwrap().push(val),
                    Err(e) => eprintln!("Error Reading File {}, Error {}", &files_clone2[i], e),
                }
            }
        });
    });
    

    Ok(())
}

fn single_lexer(data: Vec<u8>) -> ZsmTokens {
    let item: String = String::from_utf8(data).unwrap();
    let mut lexer_token = ZsmTokens::new();
    lexer_token.split_word(&item);
    lexer_token.tokenize();
    lexer_token
}

fn multi_lexer(lexer_tokens: Arc<Mutex<Vec<ZsmTokens>>>, files_data: Arc<Mutex<Vec<Vec<u8>>>>, files_len: usize) -> Result<()> {
    if files_len == 1 {
        let token = single_lexer(files_data.lock().unwrap()[0].clone());
        lexer_tokens.lock().unwrap().push(token);
        return Ok(());
    }

    let first_half_end = files_len / 2;

    thread::scope(|s| {
        let files_data_clone = Arc::clone(&files_data);
        let lexer_tokens_clone = Arc::clone(&lexer_tokens);

        // --- First Thread ---
        s.spawn(move || {
            for i in 0..first_half_end {
                let token = single_lexer(files_data_clone.lock().unwrap()[i].clone());
                lexer_tokens_clone.lock().unwrap().push(token);
            }
        });

        // --- Second Thread ---
        let files_data_clone2 = Arc::clone(&files_data);
        let lexer_tokens_clone2 = Arc::clone(&lexer_tokens);

        s.spawn(move || {
            for i in first_half_end..files_len {
                let token = single_lexer(files_data_clone2.lock().unwrap()[i].clone());
                lexer_tokens_clone2.lock().unwrap().push(token);
            }
        });
    });

    Ok(())
}
