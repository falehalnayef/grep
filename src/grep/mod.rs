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
        if self.pattern == r"\d" {
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
