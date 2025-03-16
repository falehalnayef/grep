use Token::*;

pub type Tokens = Vec<Token>;

#[derive(Debug)]
pub enum Token {
    Literal(char),
    Digit,
    AlphaNumeric,
    Group(bool, Vec<char>),
    NoToken,
}

pub struct Scanner {
    pattern: String,
    tokens: Tokens,
}

impl Scanner {
    pub fn new(pattern: String) -> Self {
        Self {
            pattern,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> &Tokens {
        self.tokenize();
        &self.tokens
    }

    fn tokenize(&mut self) {
        let mut chars = self.pattern.chars();

        while let Some(char) = chars.next() {
            let token = match char {
                '\\' => {
                    if let Some(next_char) = chars.next() {
                        match next_char {
                            'd' => Digit,
                            'w' => AlphaNumeric,
                            _ => NoToken,
                        }
                    } else {
                        NoToken
                    }
                }
                '[' => {
                    let mut positive = true;
                    let mut group_chars = Vec::new();

                    if let Some(next_char) = chars.next() {
                        if next_char == '^' {
                            positive = false;
                        } else {
                            group_chars.push(next_char);
                        }
                    }

                    while let Some(c) = chars.next() {
                        if c == ']' {
                            break;
                        }
                        group_chars.push(c);
                    }

                    Group(positive, group_chars)
                }
                _ => Literal(char),
            };

            self.tokens.push(token);
        }
    }
}
