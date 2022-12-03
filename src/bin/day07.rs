use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let wire_a_value = {
            let mut circuit = Circuit::from_lines(BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok()))?;

            let wire_a_value = *circuit.emulate().get("a").expect("Wire a should have a value");
            println!("Value of wire a: {}", wire_a_value);

            wire_a_value
        };

        // Now, take the signal you got on wire a, override wire b to that signal, and reset the
        // other wires (including wire a). What new signal is ultimately provided to wire a?
        {
            let mut circuit = Circuit::from_lines(BufReader::new(File::open(path)?)
                .lines()
                .filter_map(|line| line.ok()))?;

            circuit.inputs_by_wire.insert(
                String::from("b"),
                Input::DirectSource(Source::Signal(wire_a_value))
            );

            let more_different_wire_a_value =
                *circuit.emulate().get("a").expect("Wire a should have a value");

            println!("Value of wire a after override shenanigans: {}", more_different_wire_a_value);
        }

        Ok(())
    } else {
        Err("Usage: day07 INPUT_FILE_PATH".into())
    }
}

struct Circuit {
    inputs_by_wire: HashMap<String, Input>,
    values_by_wire: HashMap<String, u16>,
}

impl Circuit {
    fn from_lines(lines: impl Iterator<Item = String>) -> Result<Self, Box<dyn Error>> {
        let mut inputs_by_wire = HashMap::new();

        for line in lines {
            let pieces: Vec<&str> = line.split(" -> ").collect();

            if let [input, wire] = pieces.as_slice() {
                inputs_by_wire.insert(String::from(*wire), Input::from_str(input)?);
            } else {
                return Err(format!("Could not parse line: {}", line).into())
            }
        }

        Ok(Circuit { inputs_by_wire, values_by_wire: HashMap::new() })
    }

    fn emulate(&mut self) -> HashMap<String, u16> {
        // Iteratively resolve wires until we've got them all
        while self.values_by_wire.len() < self.inputs_by_wire.len() {
            for wire in self.inputs_by_wire.keys() {
                if !self.values_by_wire.contains_key(wire) {
                    if let Ok(value) = self.resolve_wire(wire) {
                        self.values_by_wire.insert(wire.clone(), value);
                    }
                }
            }
        }

        self.values_by_wire.clone()
    }

    fn resolve_wire(&self, wire: &str) -> Result<u16, ()> {
        let input = self.inputs_by_wire.get(wire).expect("Wire should have input");

        let value = match input {
            Input::DirectSource(source) => self.resolve_source(source)?,
            Input::Not(source) => !self.resolve_source(source)?,
            Input::And(a, b) => self.resolve_source(a)? & self.resolve_source(b)?,
            Input::Or(a, b) => self.resolve_source(a)? | self.resolve_source(b)?,
            Input::LeftShift(source, bits) => self.resolve_source(source)? << bits,
            Input::RightShift(source, bits) => self.resolve_source(source)? >> bits,
        };

        Ok(value)
    }

    fn resolve_source(&self, source: &Source) -> Result<u16, ()> {
        match source {
            Source::Signal(value) => Ok(*value),
            Source::Wire(wire) => self.values_by_wire.get(wire).copied().ok_or(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Input {
    DirectSource(Source),
    Not(Source),
    And(Source, Source),
    Or(Source, Source),
    LeftShift(Source, u8),
    RightShift(Source, u8),
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = string.split(' ').collect();

        match pieces.as_slice() {
            [source] => Ok(Input::DirectSource(Source::from_str(source)?)),
            ["NOT", source] => Ok(Input::Not(Source::from_str(source)?)),
            [a, "AND", b] => Ok(Input::And(Source::from_str(a)?, Source::from_str(b)?)),
            [a, "OR", b] => Ok(Input::Or(Source::from_str(a)?, Source::from_str(b)?)),
            [source, "LSHIFT", bits] => Ok(Input::LeftShift(Source::from_str(source)?, bits.parse()?)),
            [source, "RSHIFT", bits] => Ok(Input::RightShift(Source::from_str(source)?, bits.parse()?)),
            _ => Err(format!("Unparseable input string: {}", string).into())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Source {
    Signal(u16),
    Wire(String),
}

impl FromStr for Source {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.chars().all(|c| c.is_numeric()) {
            Ok(Source::Signal(string.parse()?))
        } else if string.chars().all(|c| c.is_ascii_lowercase()) {
            Ok(Source::Wire(String::from(string)))
        } else {
            Err(format!("Unparseable source string: {}", string).into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_CIRCUIT: &str = indoc! {"
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
    "};

    #[test]
    fn test_emulate_circuit() {
        let expected: HashMap<String, u16> = HashMap::from([
            (String::from("d"), 72),
            (String::from("e"), 507),
            (String::from("f"), 492),
            (String::from("g"), 114),
            (String::from("h"), 65412),
            (String::from("i"), 65079),
            (String::from("x"), 123),
            (String::from("y"), 456),
        ]);

        assert_eq!(
            expected,
            Circuit::from_lines(TEST_CIRCUIT.lines().map(String::from)).unwrap().emulate()
        );
    }
}