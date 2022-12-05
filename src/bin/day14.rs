use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let reindeer: Vec<Reindeer> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| Reindeer::from_str(&line))
            .collect::<Result<Vec<Reindeer>, _>>()?;

        let winner = reindeer.iter()
            .max_by_key(|reindeer| reindeer.distance_traveled(2503))
            .unwrap();

        println!(
            "Winning reindeer after 2503 seconds: {} at {} km",
            winner.name,
            winner.distance_traveled(2503)
        );

        Ok(())
    } else {
        Err("Usage: day14 INPUT_FILE_PATH".into())
    }
}

struct Reindeer {
    name: String,
    velocity: u32,
    fly_time: u32,
    rest_time: u32,
}

impl FromStr for Reindeer {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new("([A-Za-z]+) can fly ([0-9]+) km/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds.").unwrap();

        if let Some(captures) = pattern.captures(string) {
            let name = captures[1].to_string();
            let velocity = captures[2].parse()?;
            let fly_time = captures[3].parse()?;
            let rest_time = captures[4].parse()?;

            Ok(Reindeer { name, velocity, fly_time, rest_time })
        } else {
            Err("Could not parse line".into())
        }
    }
}

impl Reindeer {
    fn distance_traveled(&self, time: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let full_cycles = time / cycle_time;
        let travel_time_in_last_cycle = cmp::min(time % cycle_time, self.fly_time);

        ((self.fly_time * full_cycles) + travel_time_in_last_cycle) * self.velocity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance_traveled() {
        let comet = Reindeer::from_str("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.").unwrap();
        let dancer = Reindeer::from_str("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.").unwrap();

        assert_eq!(14, comet.distance_traveled(1));
        assert_eq!(16, dancer.distance_traveled(1));

        assert_eq!(140, comet.distance_traveled(10));
        assert_eq!(160, dancer.distance_traveled(10));

        assert_eq!(140, comet.distance_traveled(11));
        assert_eq!(176, dancer.distance_traveled(11));

        assert_eq!(140, comet.distance_traveled(12));
        assert_eq!(176, dancer.distance_traveled(12));

        assert_eq!(1120, comet.distance_traveled(1000));
        assert_eq!(1056, dancer.distance_traveled(1000));
    }
}
