pub mod matcher;
pub mod scanner;

use matcher::Matcher;
use scanner::Scanner;

pub struct Grep {
    pattern: String,
    data: String,
}

impl Grep {
    pub fn new(pattern: String, data: String) -> Self {
        Self { pattern, data }
    }

    pub fn match_pattern(&self) {
        let mut scanner = Scanner::new(&self.pattern);
        let tokens = scanner.scan();
        let matcher = Matcher::new(tokens);
        let (boolRes, strRes) = matcher.match_tokens(&self.data);

        println!("{}", strRes);
    }
}
