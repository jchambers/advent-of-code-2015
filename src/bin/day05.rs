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

        let nice_strings = strings.iter()
            .filter(|string| is_nice(string))
            .count();

        println!("Nice strings: {}", nice_strings);

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
}
