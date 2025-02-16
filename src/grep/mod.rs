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
            let result = self
                .data
                .chars()
                .map(|c| {
                    if c.is_numeric() {
                        format!("{}", c.to_string().green())
                    } else {
                        c.to_string()
                    }
                })
                .collect();
            Some(result)
        } else if self.data.contains(&self.pattern) {
            let colored = format!("{}", &self.pattern.green());
            Some(self.data.replace(&self.pattern, &colored))
        } else {
            None
        }
    }
}
