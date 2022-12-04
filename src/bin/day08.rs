use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let strings: Vec<String> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .collect();

        let delta: usize = strings.iter()
            .map(|string| code_characters(string) - memory_characters(string))
            .sum();

        println!("Total difference between size in code and size in memory: {}", delta);

        Ok(())
    } else {
        Err("Usage: day08 INPUT_FILE_PATH".into())
    }
}

fn code_characters(string: &str) -> usize {
    string.chars().count()
}

fn memory_characters(string: &str) -> usize {
    parse_escaped_string(string).chars().count()
}

fn parse_escaped_string(escaped_string: &str) -> String {
    debug_assert!(escaped_string.is_ascii());

    let mut string = String::new();
    let mut bytes = escaped_string.as_bytes();

    // Skip opening/closing quotes
    bytes = &bytes[1..bytes.len() - 1];

    while let Some(i) = memchr::memchr(b'\\', bytes) {
        string.push_str(std::str::from_utf8(&bytes[..i]).unwrap());

        let (escaped_character, len) = match bytes[i + 1] {
            b'\\' => ('\\', 1),
            b'"' => ('"', 1),
            b'x' => (u8::from_str_radix(std::str::from_utf8(&bytes[i + 2..=i + 3]).unwrap(), 16).unwrap() as char, 3),
            _ => panic!("Unexpected character after backslash")
        };

        string.push(escaped_character);
        bytes = &bytes[i + len + 1..];
    }

    if !bytes.is_empty() {
        string.push_str(std::str::from_utf8(bytes).unwrap());
    }

    string
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_code_characters() {
        assert_eq!(2, code_characters(r#""""#));
        assert_eq!(5, code_characters(r#""abc""#));
        assert_eq!(10, code_characters(r#""aaa\"aaa""#));
        assert_eq!(6, code_characters(r#""\x27""#));
    }

    #[test]
    fn test_memory_characters() {
        assert_eq!(0, memory_characters(r#""""#));
        assert_eq!(3, memory_characters(r#""abc""#));
        assert_eq!(7, memory_characters(r#""aaa\"aaa""#));
        assert_eq!(1, memory_characters(r#""\x27""#));
    }

    #[test]
    fn test_parse_escaped_string() {
        assert_eq!("", parse_escaped_string(r#""""#));
        assert_eq!("abc", parse_escaped_string(r#""abc""#));
        assert_eq!("aaa\"aaa", parse_escaped_string(r#""aaa\"aaa""#));
        assert_eq!("'", parse_escaped_string(r#""\x27""#));
    }
}
