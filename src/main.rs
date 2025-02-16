mod grep;

use std::{env, io};

use grep::Grep;
fn main() {
    let mut buf = String::new();
    let pattern = env::args().nth(1).expect("msg");

    io::stdin().read_line(&mut buf).expect("msg");

    let g = Grep::new(pattern, buf);

    match g.match_pattern() {
        Some(colored_string) => println!("{}", colored_string),
        None => println!("Pattern not found!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::*;

    #[test]
    fn test_digit_matching() {
        // Digits should be colored green
        let grep = Grep::new(r"\d".to_string(), "a1b2c".to_string());
        let result = grep.match_pattern().unwrap();
        assert_eq!(
            result,
            format!(
                "a{}b{}c",
                "1".green(), // First digit
                "2".green()  // Second digit
            )
        );
    }

    #[test]
    fn test_no_digits() {
        // Should return None when no digits found
        let grep = Grep::new(r"\d".to_string(), "apple".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_string_pattern() {
        // All occurrences should be green
        let grep = Grep::new("na".to_string(), "banana".to_string());
        let result = grep.match_pattern().unwrap();
        assert_eq!(
            result,
            format!("ba{}{}", "na".red().green(), "na".red().green())
        );
    }

    #[test]
    fn test_case_sensitivity() {
        // Default is case-sensitive
        let grep = Grep::new("A".to_string(), "apple".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_no_match() {
        // No match → return None
        let grep = Grep::new("xyz".to_string(), "apple".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_empty_input() {
        // Empty input → no match
        let grep = Grep::new("a".to_string(), "".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_special_char_pattern() {
        // Should handle regex special chars as literals
        let grep = Grep::new(r".".to_string(), "a.b".to_string());
        assert_eq!(grep.match_pattern().unwrap(), format!("a{}b", ".".green()));
    }

    #[test]
    fn test_overlapping_matches() {
        // Non-overlapping matches only (due to String::replace behavior)
        let grep = Grep::new("aa".to_string(), "aaaa".to_string());
        assert_eq!(
            grep.match_pattern().unwrap(),
            format!("{}{}", "aa".green(), "aa".green())
        );
    }
}
