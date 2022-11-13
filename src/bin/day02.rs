use std::{env, error};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let file = File::open(path)?;

        let presents: Vec<Present> = BufReader::new(file).lines()
            .filter_map(|line| line.ok())
            .filter_map(|line| Present::from_str(&line).ok())
            .collect();

        let required_wrapping_paper: u32 = presents.iter()
            .map(|present| present.required_wrapping_paper())
            .sum();

        println!("Total required wrapping paper: {} square feet", required_wrapping_paper);

        let required_ribbon: u32 = presents.iter()
            .map(|present| present.required_ribbon())
            .sum();

        println!("Total required ribbon: {} linear feet", required_ribbon);

        Ok(())
    } else {
        Err("Usage: day02 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Present {
    dimensions: [u32; 3]
}

impl Present {
    pub fn required_wrapping_paper(&self) -> u32 {
        let smallest_side = *[self.dimensions[0] * self.dimensions[1],
            self.dimensions[1] * self.dimensions[2],
            self.dimensions[0] * self.dimensions[2]].iter().min().unwrap();

        let surface_area = 2 * ((self.dimensions[0] * self.dimensions[1]) +
            (self.dimensions[1] * self.dimensions[2]) +
            (self.dimensions[0] * self.dimensions[2]));

        surface_area + smallest_side
    }

    pub fn required_ribbon(&self) -> u32 {
        let smallest_perimeter = 2 * *[self.dimensions[0] + self.dimensions[1],
            self.dimensions[1] + self.dimensions[2],
            self.dimensions[0] + self.dimensions[2]].iter().min().unwrap();

        let volume = self.dimensions[0] * self.dimensions[1] * self.dimensions[2];

        smallest_perimeter + volume
    }
}

impl FromStr for Present {
    type Err = Box<dyn error::Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(Self { dimensions: string.split('x')
            .filter_map(|component| component.parse::<u32>().ok())
            .collect::<Vec<u32>>()
            .try_into()
            .map_err(|_| "Could not parse string as dimensions")?
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_present_from_string() {
        assert_eq!(Present { dimensions: [1, 12, 3] }, Present::from_str("1x12x3").unwrap());
    }

    #[test]
    fn test_required_wrapping_paper() {
        assert_eq!(58, Present { dimensions: [2, 3, 4] }.required_wrapping_paper());
        assert_eq!(43, Present { dimensions: [1, 1, 10] }.required_wrapping_paper());
    }

    #[test]
    fn test_required_ribbon() {
        assert_eq!(34, Present { dimensions: [2, 3, 4] }.required_ribbon());
        assert_eq!(14, Present { dimensions: [1, 1, 10] }.required_ribbon());
    }
}
