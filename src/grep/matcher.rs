use super::scanner::{Token::*, Tokens};
use std::char;

pub struct Matcher<'a> {
    tokens: &'a Tokens,
}

impl<'a> Matcher<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        Self { tokens }
    }

    pub fn match_tokens(&self, input_data: String) -> bool {
        let chars: Vec<char> = input_data.chars().collect();
        let mut counter = 0;

        for token in self.tokens {
            if counter >= chars.len() {
                return false;
            }

            match token {
                Literal(c) => {
                    if !Self::match_literal(*c, chars[counter]) {
                        return false;
                    }
                    println!("{} = {}", c, chars[counter]);
                }
                Digit => {
                    if !Self::match_digit(chars[counter]) {
                        return false;
                    }

                    println!("is Digit {}", chars[counter]);
                }
                AlphaNumeric => {
                    if !Self::match_alpha_numeric(chars[counter]) {
                        return false;
                    }

                    println!("is alphanumeric {}", chars[counter]);
                }
                Group(positive, pattern_chars) => {
                    let matched =
                        Self::match_chars_group(*positive, pattern_chars, &chars[counter..]);

                    if matched == 0 {
                        return false;
                    }

                    println!(
                        "{:?} = {:?}",
                        pattern_chars,
                        &chars[counter..counter + matched]
                    );
                    counter += matched - 1; // because it will increment by 1 at the end of the loop
                }

                NoToken => {
                    continue;
                }
            }

            counter += 1;
        }

        true
    }

    fn match_digit(c: char) -> bool {
        c.is_numeric()
    }

    fn match_alpha_numeric(c: char) -> bool {
        c.is_alphanumeric()
    }

    fn match_literal(c1: char, c2: char) -> bool {
        c1 == c2
    }

    fn match_chars_group(positive: bool, pattern_chars: &Vec<char>, input_chars: &[char]) -> usize {
        let mut matched = 0;

        for &c in input_chars {
            let contains = pattern_chars.contains(&c);
            if (positive && contains) || (!positive && !contains) {
                matched += 1;
            } else {
                break;
            }
        }

        matched
    }
}
