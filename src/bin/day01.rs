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

#[cfg(test)]
mod test {
    use crate::get_floor;

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
}
