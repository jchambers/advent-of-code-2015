extern crate core;

use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::AddAssign;
use crate::Move::{East, North, South, West};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let moves: Vec<Move> = {
            let mut file = File::open(path)?;
            let mut directions = String::new();

            file.read_to_string(&mut directions)?;

            directions.chars()
                .map(Move::try_from)
                .collect::<Result<Vec<Move>, Box<dyn Error>>>()?
        };

        println!("Distinct houses visited by single actor: {}", distinct_houses_visited(&moves, 1));
        println!("Distinct houses visited by two actors: {}", distinct_houses_visited(&moves, 2));

        Ok(())
    } else {
        Err("Usage: day03 INPUT_FILE_PATH".into())
    }
}

enum Move {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Move {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '^' => Ok(North),
            'v' => Ok(South),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err("Illegal direction".into())
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

impl AddAssign<&Move> for Position {
    fn add_assign(&mut self, rhs: &Move) {
        match rhs {
            North => self.1 += 1,
            South => self.1 -= 1,
            East => self.0 += 1,
            West => self.0 -= 1,
        };
    }
}

fn distinct_houses_visited(moves: &[Move], actors: usize) -> u32 {
    let mut visited_houses = HashSet::new();

    for actor in 0..actors {
        visited_houses.extend(
            moves.iter()
                .skip(actor)
                .step_by(actors)
                .scan(Position(0, 0), |position, mov| {
                    *position += mov;

                    Some(*position)
                }));
    }

    visited_houses.insert(Position(0, 0));
    visited_houses.len() as u32
}

#[cfg(test)]
mod test {
    use crate::distinct_houses_visited;
    use crate::Move::{East, North, South, West};

    #[test]
    fn test_distinct_houses_visited() {
        assert_eq!(2, distinct_houses_visited(&[East], 1));
        assert_eq!(4, distinct_houses_visited(&[North, East, South, West], 1));
        assert_eq!(2, distinct_houses_visited(
            &[North, South, North, South, North, South, North, South, North, South], 1));

        assert_eq!(3, distinct_houses_visited(&[North, South], 2));
        assert_eq!(3, distinct_houses_visited(&[North, East, South, West], 2));
        assert_eq!(11, distinct_houses_visited(
            &[North, South, North, South, North, South, North, South, North, South], 2));
    }
}
