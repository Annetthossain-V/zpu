
pub struct ZsmTokens {
    split_word: Vec<String>,
    tokens: Vec<Token<Keys, String>>,
}

impl ZsmTokens {
    pub fn new() -> Self {
        ZsmTokens {
            split_word: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn split_word(&mut self, item: &str) {
        let tokenized = word_split(item);
        self.split_word = tokenized;
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
                "sptr" => self.tokens.push(Tokens::Key(Keys::Sptr)),

                _ => self.tokens.push(Token::Value(self.split_word[i].clone())),
            }
        }
    }
}

fn word_split(item: &str) -> Vec<String> {
    let delims = &[' ', ',', ':', '\n'];
    item.split(delims)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

enum Token<T, U> {
    Key(T),
    Value(U),
}

enum Keys {
    Section,
    Func,
    Mov,
    End,
    Alloc,
    Dealloc,
    Lptr,
    Sptr,
}

