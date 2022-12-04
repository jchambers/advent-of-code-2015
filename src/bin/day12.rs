use std::error::Error;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let json = fs::read_to_string(path)?;

        println!("Sum of numbers in JSON text: {}", sum_of_numbers(&json));

        Ok(())
    } else {
        Err("Usage: day12 INPUT_FILE_PATH".into())
    }
}

fn sum_of_numbers(json: &str) -> i64 {
    lazy_static! {
        static ref NUMBER_PATTERN: Regex = Regex::new("(-?[0-9]+)").unwrap();
    }

    NUMBER_PATTERN.captures_iter(json)
        .map(|capture| capture[1].parse::<i64>().unwrap())
        .sum()
}

#[cfg(test)]
mod test {
    use crate::sum_of_numbers;

    #[test]
    fn test_sum_of_numbers() {
        assert_eq!(6, sum_of_numbers(r#"[1,2,3]"#));
        assert_eq!(6, sum_of_numbers(r#"{"a":2,"b":4}"#));
        assert_eq!(3, sum_of_numbers(r#"[[[3]]]"#));
        assert_eq!(3, sum_of_numbers(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(0, sum_of_numbers(r#"{"a":[-1,1]}"#));
        assert_eq!(0, sum_of_numbers(r#"[-1,{"a":1}]"#));
        assert_eq!(0, sum_of_numbers(r#"[]"#));
        assert_eq!(0, sum_of_numbers(r#"{}"#));
    }
}
