use std::error::Error;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(seed) = args.get(1) {
        let mut evolved = seed.clone();

        for _ in 0..40 {
            evolved = evolve(&evolved);
        }

        println!("Length after 40 iterations of {}: {}", seed, evolved.len());

        for _ in 0..10 {
            evolved = evolve(&evolved);
        }

        println!("Length after 50 iterations of {}: {}", seed, evolved.len());

        Ok(())
    } else {
        Err("Usage: day10 SEED".into())
    }
}

fn evolve(string: &str) -> String {
    let mut evolved = String::new();

    for (char, group) in &string.chars().group_by(|c| *c) {
        evolved.push_str(format!("{}", group.count()).as_str());
        evolved.push(char);
    }

    evolved
}

#[cfg(test)]
mod test {
    use crate::evolve;

    #[test]
    fn test_evolve() {
        assert_eq!("11", evolve("1"));
        assert_eq!("21", evolve("11"));
        assert_eq!("1211", evolve("21"));
        assert_eq!("111221", evolve("1211"));
        assert_eq!("312211", evolve("111221"));
    }
}
