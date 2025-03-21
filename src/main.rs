mod grep;

use grep::{scanner::Scanner, scanner::Tokens, Grep};
use std::{env, io, time};
fn main() {
    let time = time::Instant::now();
    let pattern = String::from(r"\d [a-zA-Z] [^abc] x");
    let mut scanner = Scanner::new(pattern);
    let tokens: &Tokens = scanner.scan();

    for token in tokens {
        println!("{:?}", token);
    }
    // println!("{:?}", time.elapsed());

    // let mut buf = String::new();
    // let pattern = env::args().nth(1).expect("failed getting the arg");

    // io::stdin()
    //     .read_line(&mut buf)
    //     .expect("failed reading the input");

    // let g = Grep::new(pattern, buf);

    // match g.match_pattern() {
    //     Some(colored_string) => println!("{}", colored_string),
    //     None => println!("Pattern not found!"),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

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

    #[test]
    fn test_alphanumeric_match() {
        // Digits and chars should be colored green
        let grep = Grep::new(r"\w".to_string(), "a-1".to_string());
        let out = format!("{}-{}", "a".green(), "1".green());
        assert_eq!(grep.match_pattern().unwrap(), out);
    }

    #[test]
    fn test_no_alphanumeric_match() {
        // special chars should not be colored green

        let grep = Grep::new(r"\w".to_string(), "!-#$%^&".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_positive_chars_match() {
        // Digits and chars that in [] should be colored green
        let grep = Grep::new("[ea]".to_string(), "apple".to_string());
        let out = format!("{}ppl{}", "a".green(), "e".green());
        assert_eq!(grep.match_pattern().unwrap(), out);
    }

    #[test]
    fn test_no_positive_chars_match() {
        // Digits and chars that are not in [] should be not colored green
        let grep = Grep::new("[m]".to_string(), "apple".to_string());
        assert!(grep.match_pattern().is_none());
    }

    #[test]
    fn test_negative_chars_match() {
        // Digits and chars that in [^] should not be colored green
        let grep = Grep::new("[^ea]".to_string(), "apple".to_string());
        let out = format!("a{}{}{}e", "p".green(), "p".green(), "l".green());
        assert_eq!(grep.match_pattern().unwrap(), out);
    }

    #[test]
    fn test_no_negative_chars_match() {
        // Digits and chars that are not in [^] should be colored green
        let grep = Grep::new("[^m]".to_string(), "app".to_string());
        let out = format!("{}{}{}", "a".green(), "p".green(), "p".green());
        assert_eq!(grep.match_pattern().unwrap(), out);
    }
}
