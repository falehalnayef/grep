use colored::Colorize;

pub struct Grep {
    pattern: String,
    data: String,
}

impl Grep {
    pub fn new(pattern: String, data: String) -> Self {
        Self { pattern, data }
    }

    pub fn match_pattern(&self) -> Option<String> {
        if self.pattern.starts_with("[^") && self.pattern.ends_with(']') && self.pattern.len() > 3 {
            let mut it_has = false;

            let pattern_chars: Vec<char> = self.pattern.chars().collect();

            let result: String = self
                .data
                .chars()
                .map(|c| {
                    if !pattern_chars.contains(&c) {
                        if !it_has {
                            it_has = true
                        };
                        format!("{}", c.to_string().green())
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            if it_has {
                Some(result)
            } else {
                None
            }
        } else if self.pattern.starts_with('[') && self.pattern.ends_with(']') {
            let mut it_has_alphanum = false;

            let pattern_chars: Vec<char> = self.pattern.chars().collect();

            let result: String = self
                .data
                .chars()
                .map(|c| {
                    if pattern_chars.contains(&c) {
                        if !it_has_alphanum {
                            it_has_alphanum = true
                        };
                        format!("{}", c.to_string().green())
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            if it_has_alphanum {
                Some(result)
            } else {
                None
            }
        } else if self.pattern == r"\w" {
            let mut it_has_alphanum = false;
            let result: String = self
                .data
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() {
                        if !it_has_alphanum {
                            it_has_alphanum = true
                        };
                        format!("{}", c.to_string().green())
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            if it_has_alphanum {
                Some(result)
            } else {
                None
            }
        } else if self.pattern == r"\d" {
            let mut it_has_num = false;
            let result: String = self
                .data
                .chars()
                .map(|c| {
                    if c.is_numeric() {
                        if !it_has_num {
                            it_has_num = true
                        };
                        format!("{}", c.to_string().green())
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            if it_has_num {
                Some(result)
            } else {
                None
            }
        } else if self.data.contains(&self.pattern) {
            let colored = format!("{}", &self.pattern.green());
            Some(self.data.replace(&self.pattern, &colored))
        } else {
            None
        }
    }
}
