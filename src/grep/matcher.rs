use super::scanner::{Token, Tokens};
use colored::Colorize;

pub struct Matcher<'a> {
    tokens: &'a Tokens,
}

impl<'a> Matcher<'a> {
    pub fn new(tokens: &'a Tokens) -> Self {
        Self { tokens }
    }

    pub fn match_tokens(&self, input_data: &String) -> (bool, String) {
        let chars: Vec<char> = input_data.chars().collect();
        let mut res = String::new();
        let mut counter = 0;

        println!("tokens: {:?}", self.tokens);

        // === Case: Only one Digit token — highlight all digits
        if self.tokens.len() == 1 && matches!(self.tokens[0], Token::Digit) {
            let mut found_digit = false;

            let highlighted: String = chars
                .iter()
                .map(|c| {
                    if c.is_ascii_digit() {
                        found_digit = true;
                        c.to_string().red().to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect();

            return if found_digit {
                (true, highlighted)
            } else {
                (false, "None".to_string())
            };
        }

        // === Case: Only one AlphaNumeric token — highlight all alphanumerics
        if self.tokens.len() == 1 && matches!(self.tokens[0], Token::AlphaNumeric) {
            let mut found_alnum = false;

            let highlighted: String = chars
                .iter()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        found_alnum = true;
                        c.to_string().red().to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect();

            return if found_alnum {
                (true, highlighted)
            } else {
                (false, "None".to_string())
            };
        }

        // === Case: Only one Group token — match all chars according to group rules
        if self.tokens.len() == 1 {
            if let Token::Group(positive, pattern_chars) = &self.tokens[0] {
                let mut found = false;

                let highlighted: String = chars
                    .iter()
                    .map(|c| {
                        let in_group = pattern_chars.contains(c);
                        let should_color = (*positive && in_group) || (!*positive && !in_group);

                        if should_color {
                            found = true;
                            c.to_string().red().to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect();

                return if found {
                    (true, highlighted)
                } else {
                    (false, "None".to_string())
                };
            }
        }

        // === Case: One literal — highlight all its appearances
        let literals: Vec<char> = self
            .tokens
            .iter()
            .filter_map(|t| {
                if let Token::Literal(c) = t {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect();

        if literals.len() == 1
            && self
                .tokens
                .iter()
                .all(|t| matches!(t, Token::Literal(_) | Token::NoToken))
        {
            let lit = literals[0];
            let mut found = false;

            for &c in &chars {
                if c == lit {
                    found = true;
                    res.push_str(&c.to_string().red().to_string());
                } else {
                    res.push(c);
                }
            }

            return if found {
                (true, res)
            } else {
                (false, "None".to_string())
            };
        }

        // === Case: Ordered literals — must be adjacent
        if literals.len() > 1
            && self
                .tokens
                .iter()
                .all(|t| matches!(t, Token::Literal(_) | Token::NoToken))
        {
            let input_str: String = chars.iter().collect();
            let pattern: String = literals.iter().collect();

            if let Some(pos) = input_str.find(&pattern) {
                let before = &input_str[..pos];
                let matched = &input_str[pos..pos + pattern.len()];
                let after = &input_str[pos + pattern.len()..];

                res.push_str(before);
                for c in matched.chars() {
                    res.push_str(&c.to_string().red().to_string());
                }
                res.push_str(after);

                return (true, res);
            } else {
                return (false, "None".to_string());
            }
        }

        // === General token stream match (strict order)
        let mut token_index = 0;

        while counter < chars.len() && token_index < self.tokens.len() {
            match &self.tokens[token_index] {
                Token::Literal(expected_char) => {
                    if chars[counter] == *expected_char {
                        res.push_str(&chars[counter].to_string().red().to_string());
                        token_index += 1;
                    } else {
                        return (false, "None".to_string());
                    }
                }

                Token::Digit => {
                    if chars[counter].is_ascii_digit() {
                        res.push_str(&chars[counter].to_string().red().to_string());
                        token_index += 1;
                    } else {
                        return (false, "None".to_string());
                    }
                }

                Token::AlphaNumeric => {
                    if chars[counter].is_ascii_alphanumeric() {
                        res.push_str(&chars[counter].to_string().red().to_string());
                        token_index += 1;
                    } else {
                        return (false, "None".to_string());
                    }
                }

                Token::Group(positive, pattern_chars) => {
                    let (matched, colored_group) =
                        Self::match_chars_group(*positive, pattern_chars, &chars[counter..]);

                    if matched == 0 {
                        return (false, "None".to_string());
                    }

                    res.push_str(&colored_group);
                    counter += matched;
                    token_index += 1;
                    continue;
                }

                Token::NoToken => {
                    res.push(chars[counter]);
                    // Skip character but don't consume token
                }
            }

            counter += 1;
        }

        if token_index < self.tokens.len() {
            return (false, "None".to_string());
        }

        // Append the rest
        while counter < chars.len() {
            res.push(chars[counter]);
            counter += 1;
        }

        (true, res)
    }

    fn match_chars_group(
        positive: bool,
        pattern_chars: &Vec<char>,
        input_chars: &[char],
    ) -> (usize, String) {
        let mut matched = 0;
        let mut res = String::new();

        for &c in input_chars {
            let contains = pattern_chars.contains(&c);
            if (positive && contains) || (!positive && !contains) {
                matched += 1;
                res.push_str(&c.to_string().red().to_string());
            } else {
                break;
            }
        }

        (matched, res)
    }
}
