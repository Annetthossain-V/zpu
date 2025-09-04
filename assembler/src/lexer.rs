#![allow(unused)]

pub struct ZsmTokens {
    pub split_word: Vec<String>,
    pub tokens: Vec<Token<Keys, String>>,
}

impl ZsmTokens {
    pub fn new() -> Self {
        ZsmTokens {
            split_word: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn split_word(&mut self, item: &str) {
        let delims = &[' ', ',', '$', ':', '\n']; 
        let mut current = String::new();
        let mut in_quotes = false;

        self.split_word.clear();
        self.tokens.clear();

        for c in item.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(c);
                }
                d if delims.contains(&d) && !in_quotes => {
                    if !current.is_empty() {
                        self.split_word.push(current.trim().to_string());
                        current.clear();
                    }
                    if d != ' ' && d != '\n' {
                        self.split_word.push(d.to_string());
                    }
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            self.split_word.push(current.trim().to_string());
        }
    }

    pub fn tokenize(&mut self) {
        for i in 0..self.split_word.len() { 
            match self.split_word[i].as_str() {
                "section" => self.tokens.push(Token::Key(Keys::Section)),
                "func" => self.tokens.push(Token::Key(Keys::Func)),
                "end" => self.tokens.push(Token::Key(Keys::End)),
                "mov" => self.tokens.push(Token::Key(Keys::Mov)),
                "alloc" => self.tokens.push(Token::Key(Keys::Alloc)),
                "lptr" => self.tokens.push(Token::Key(Keys::Lptr)),
                "sptr" => self.tokens.push(Token::Key(Keys::Sptr)),

                _ => self.tokens.push(Token::Value(self.split_word[i].clone())),
            }
        }
    }
}

pub enum Token<T, U> {
    Key(T),
    Value(U),
}

pub enum Keys {
    Section,
    Func,
    Mov,
    End,
    Alloc,
    Dealloc,
    Lptr,
    Sptr,
}

