use std::env::args;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

#[allow(unused)]
pub struct Flags {
    pub files: Vec<String>,
    pub options: Vec<Options>,
    info: bool,
}

#[allow(unused)]
pub enum Options {
    Help,
    Version,
}

#[allow(unused)]
impl Flags {
    pub fn new() -> Self {
        Flags {
            files: Vec::new(),
            options: Vec::new(),
            info: false,
        }
    }

    pub fn parse(&mut self) -> Result<()> {
        let args: Vec<String> = args().skip(1).collect();
        if (args.len() == 0) {
            self.options.push(Options::Help);
            self.info = true;
        }

        for arg in args {
            if (arg.chars().nth(0).unwrap() == '-') {
                match arg.as_str() {
                    "-h" | "--help" => {
                        self.options.push(Options::Help);
                        self.info = true;
                    }
                    "-v" | "--version" => {
                        self.options.push(Options::Version);
                        self.info = true;
                    }
                    _ => panic!("Unknown Option {}", arg),
                }
            } else {
                let file = Path::new(&arg);
                if (!file.exists()) {
                    return Err(Error::new(ErrorKind::NotFound, "File not found"));
                }
                if (file.extension().unwrap() != "zob") {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid file extension",
                    ));
                }
                self.files.push(arg);
            }
        }

        if (self.files.len() == 0 && !self.info) {
            return Err(Error::new(ErrorKind::InvalidInput, "No files specified"));
        }

        Ok(())
    }

    pub fn info(&self) {
        if (self.info) {
            for opt in &self.options {
                match opt {
                    Options::Help => println!("{HELP}"),
                    Options::Version => println!("{VERSION}"),
                    _ => continue,
                }
            }
            std::process::exit(0);
        }
    }
}

const VERSION: &str = "zpu runtime version 0.1";
const HELP: &str = r#"usage <Options> <files>
Options:
 --help     # prints this message
 --version  # prints version info
"#;
