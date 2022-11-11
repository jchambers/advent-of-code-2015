use std::{env, error};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let mut file = File::open(path)?;
        let mut directions = String::new();

        file.read_to_string(&mut directions)?;

        println!("Destination floor: {}", get_floor(&directions));
        println!("First position leading to basement: {}", find_first_basement_position(&directions));

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

fn get_floor(directions: &str) -> i32 {
    directions.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        })
        .sum()
}

fn find_first_basement_position(directions: &str) -> usize {
    let mut current_floor = 0;

    for (i, c) in directions.chars().enumerate() {
        current_floor += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };

        if current_floor < 0 {
            return i + 1;
        }
    }

    panic!()
}

#[cfg(test)]
mod test {
    use crate::{find_first_basement_position, get_floor};

    #[test]
    fn test_get_floor() {
        assert_eq!(0, get_floor("(())"));
        assert_eq!(0, get_floor("()()"));
        assert_eq!(3, get_floor("((("));
        assert_eq!(3, get_floor("(()(()("));
        assert_eq!(3, get_floor("))((((("));
        assert_eq!(-1, get_floor("())"));
        assert_eq!(-1, get_floor("))("));
        assert_eq!(-3, get_floor(")))"));
        assert_eq!(-3, get_floor(")())())"));
    }

    #[test]
    fn test_find_first_basement_position() {
        assert_eq!(1, find_first_basement_position(")"));
        assert_eq!(5, find_first_basement_position("()())"));
    }
}
