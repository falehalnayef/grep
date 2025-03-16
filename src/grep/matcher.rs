use super::scanner::Token;

pub fn match_tokens(tokens: &Token, input_data: String) {}

pub fn match_digit(c: char) -> bool {
    c.is_numeric()
}

pub fn match_alpha_numeric(c: char) -> bool {
    c.is_alphanumeric()
}

pub fn match_chars_group(group: Token, input_data: String) -> bool {
    if let Token::Group(positive, pattern_chars) = group {
        let mut data_chars = input_data.chars();

        let mut i = 0;

        if positive {
            while i < pattern_chars.len() {
                if !data_chars.any(|c| c == *pattern_chars.get(i).expect("failed to get char")) {
                    return false;
                }

                i += 1;
            }
            true
        } else {
            while i < pattern_chars.len() {
                if data_chars.any(|c| c == *pattern_chars.get(i).expect("failed to get char")) {
                    return false;
                }

                i += 1;
            }
            true
        }
    } else {
        false
    }
}
