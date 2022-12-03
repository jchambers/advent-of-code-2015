use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let instructions: Vec<Instruction> = BufReader::new(file).lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Instruction::from_str(&line).ok())
            .collect();

        let mut light_grid = LightGrid::new(1000, 1000);

        instructions.iter()
            .for_each(|instruction| light_grid.apply(instruction));

        println!("Lights on: {}", light_grid.lights_on());

        Ok(())
    } else {
        Err("Usage: day06 INPUT_FILE_PATH".into())
    }
}
struct LightGrid {
    lights: Vec<bool>,

    width: usize,
    height: usize,
}

impl LightGrid {
    fn new(width: usize, height: usize) -> Self {
        LightGrid {
            lights: vec![false; width * height],
            width,
            height
        }
    }

    fn apply(&mut self, instruction: &Instruction) {
        debug_assert!(instruction.start.0 <= instruction.end.0);
        debug_assert!(instruction.start.1 <= instruction.end.1);
        debug_assert!(instruction.end.0 < self.width);
        debug_assert!(instruction.end.1 < self.height);

        for y in instruction.start.1..=instruction.end.1 {
            for x in instruction.start.0..=instruction.end.0 {
                let index = (y * self.width) + x;

                self.lights[index] = match instruction.operation {
                    Operation::On => true,
                    Operation::Off => false,
                    Operation::Toggle => !self.lights[index]
                };
            }
        }
    }

    fn lights_on(&self) -> u32 {
        self.lights.iter()
            .filter(|&&light| light)
            .count() as u32
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Position(usize, usize);

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    On,
    Off,
    Toggle
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "turn on" => Ok(Operation::On),
            "turn off" => Ok(Operation::Off),
            "toggle" => Ok(Operation::Toggle),
            _ => Err("Unrecognized operation".into())
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    operation: Operation,
    start: Position,
    end: Position,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCTION_REGEX: Regex =
                Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }

        if let Some(captures) = INSTRUCTION_REGEX.captures(s) {
            let operation = Operation::from_str(&captures[1])?;
            let start = Position(captures[2].parse()?, captures[3].parse()?);
            let end = Position(captures[4].parse()?, captures[5].parse()?);

            Ok(Instruction { operation, start, end })
        } else {
            Err("Could not parse instruction".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_string() {
        assert_eq!(
            Instruction { operation: Operation::On, start: Position(0, 0), end: Position(999, 999) },
            Instruction::from_str("turn on 0,0 through 999,999").unwrap()
        );

        assert_eq!(
            Instruction { operation: Operation::Toggle, start: Position(0, 0), end: Position(999, 0) },
            Instruction::from_str("toggle 0,0 through 999,0").unwrap()
        );

        assert_eq!(
            Instruction { operation: Operation::Off, start: Position(499, 499), end: Position(500, 500) },
            Instruction::from_str("turn off 499,499 through 500,500").unwrap()
        );
    }

    #[test]
    fn test_apply_lights_on() {
        let mut light_grid = LightGrid::new(1000, 1000);
        assert_eq!(0, light_grid.lights_on());

        light_grid.apply(&Instruction { operation: Operation::On, start: Position(0, 0), end: Position(999, 999) });
        assert_eq!(1_000_000, light_grid.lights_on());

        light_grid.apply(&Instruction { operation: Operation::Toggle, start: Position(0, 0), end: Position(999, 0) });
        assert_eq!(999_000, light_grid.lights_on());

        light_grid.apply(&Instruction { operation: Operation::Toggle, start: Position(0, 0), end: Position(999, 0) });
        assert_eq!(1_000_000, light_grid.lights_on());

        light_grid.apply(&Instruction { operation: Operation::Off, start: Position(499, 499), end: Position(500, 500) });
        assert_eq!(999_996, light_grid.lights_on());
    }
}
