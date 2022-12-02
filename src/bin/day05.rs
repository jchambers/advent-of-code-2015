use std::{env, error, io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let strings: Vec<String> = BufReader::new(file).lines()
            .collect::<io::Result<Vec<String>>>()?;

        println!("Nice strings: {}", strings.iter()
            .filter(|string| is_nice(string))
            .count());

        println!("More different nice strings: {}", strings.iter()
            .filter(|string| more_different_is_nice(string))
            .count());

        Ok(())
    } else {
        Err("Usage: day05 INPUT_FILE_PATH".into())
    }
}

fn is_nice(string: &str) -> bool {
    const FORBIDDEN_SUBSTRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let vowels = string.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count();

    if vowels < 3 {
        return false;
    }

    let has_repeated_character = string.chars()
        .tuple_windows()
        .any(|(a, b)| a == b);

    if !has_repeated_character {
        return false;
    }

    let has_forbidden_substring = FORBIDDEN_SUBSTRINGS.iter()
        .any(|forbidden_substring| string.contains(forbidden_substring));

    if has_forbidden_substring {
        return false;
    }

    true
}

fn more_different_is_nice(string: &str) -> bool {
    has_repeated_non_overlapping_pair(string) && has_straddling_pair(string)
}

fn has_repeated_non_overlapping_pair(string: &str) -> bool {
    for offset in 0..string.len() - 3 {
        let substring = &string[offset..];
        let needle = &substring[0..2];
        let haystack = &substring[2..];

        if haystack.contains(needle) {
            return true;
        }
    }

    false
}

fn has_straddling_pair(string: &str) -> bool {
    string.chars()
        .tuple_windows()
        .any(|(a, _, b)| a == b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_has_repeated_non_overlapping_pair() {
        assert!(has_repeated_non_overlapping_pair("xyxy"));
        assert!(has_repeated_non_overlapping_pair("aabcdefgaa"));
        assert!(!has_repeated_non_overlapping_pair("aaa"));
    }

    #[test]
    fn test_has_straddling_pair() {
        assert!(has_straddling_pair("xyx"));
        assert!(has_straddling_pair("abcdefeghi"));
        assert!(has_straddling_pair("aaa"));
        assert!(!has_straddling_pair("nope"));
    }

    #[test]
    fn test_more_different_is_nice() {
        assert!(more_different_is_nice("qjhvhtzxzqqjkmpb"));
        assert!(more_different_is_nice("xxyxx"));
        assert!(!more_different_is_nice("uurcxstgmygtbstg"));
        assert!(!more_different_is_nice("ieodomkazucvgmuy"));
    }
}
