extern crate core;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut seating_arrangement = SeatingArrangement::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Happiness change with optimal seating arrangement: {}",
            seating_arrangement.change_in_happiness(seating_arrangement.optimal_arrangement().as_slice())
        );

        seating_arrangement.add_host();

        println!(
            "Happiness change with optimal seating arrangement (including host): {}",
            seating_arrangement.change_in_happiness(seating_arrangement.optimal_arrangement().as_slice())
        );

        Ok(())
    } else {
        Err("Usage: day13 INPUT_FILE_PATH".into())
    }
}

struct SeatingArrangement {
    happiness_changes: HashMap<String, HashMap<String, i32>>,
}

impl FromStr for SeatingArrangement {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new("([a-zA-Z]+) would (gain|lose) ([0-9]+) happiness units by sitting next to ([a-zA-Z]+).").unwrap();
        let mut happiness_changes = HashMap::new();

        for line in string.lines() {
            if let Some(captures) = pattern.captures(line) {
                let subject = captures[1].to_string();
                let is_gain = &captures[2] == "gain";
                let magnitude: i32 = captures[3].parse()?;
                let neighbor = captures[4].to_string();

                happiness_changes.entry(subject)
                    .or_insert_with(HashMap::new)
                    .insert(neighbor, if is_gain {
                        magnitude
                    } else {
                        -magnitude
                    });
            } else {
                return Err("Rule string did not match expected pattern".into());
            }
        }

        Ok(SeatingArrangement { happiness_changes })
    }
}

impl SeatingArrangement {
    fn guests(&self) -> impl Iterator<Item = &str> {
        self.happiness_changes.keys().map(|guest| guest.as_str())
    }

    fn optimal_arrangement(&self) -> Vec<&str> {
        self.guests()
            .permutations(self.happiness_changes.len())
            .max_by_key(|arrangement| self.change_in_happiness(arrangement))
            .unwrap()
    }

    fn change_in_happiness(&self, arrangement: &[&str]) -> i32 {
        assert!(arrangement.len() > 1);

        let mut happiness_change = 0;

        // Treat the first entry as a special case to deal with negative wrapping
        happiness_change += self.happiness_changes.get(arrangement[0]).unwrap().get(arrangement[1]).unwrap();
        happiness_change += self.happiness_changes.get(arrangement[0]).unwrap().get(arrangement[arrangement.len() - 1]).unwrap();


        for i in 1..arrangement.len() {
            happiness_change += self.happiness_changes.get(arrangement[i]).unwrap().get(arrangement[(i + 1) % arrangement.len()]).unwrap();
            happiness_change += self.happiness_changes.get(arrangement[i]).unwrap().get(arrangement[i - 1]).unwrap();
        }

        happiness_change
    }

    fn add_host(&mut self) {
        const HOST: &str = "Host";

        let guests: Vec<String> = self.guests().map(String::from).collect();

        for guest in guests {
            let guest = guest.to_string();

            self.happiness_changes.entry(HOST.to_string()).or_insert_with(HashMap::new).insert(guest.clone(), 0);
            self.happiness_changes.entry(guest).or_insert_with(HashMap::new).insert(HOST.to_string(), 0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_RULES: &str = indoc!{"
        Alice would gain 54 happiness units by sitting next to Bob.
        Alice would lose 79 happiness units by sitting next to Carol.
        Alice would lose 2 happiness units by sitting next to David.
        Bob would gain 83 happiness units by sitting next to Alice.
        Bob would lose 7 happiness units by sitting next to Carol.
        Bob would lose 63 happiness units by sitting next to David.
        Carol would lose 62 happiness units by sitting next to Alice.
        Carol would gain 60 happiness units by sitting next to Bob.
        Carol would gain 55 happiness units by sitting next to David.
        David would gain 46 happiness units by sitting next to Alice.
        David would lose 7 happiness units by sitting next to Bob.
        David would gain 41 happiness units by sitting next to Carol.
    "};

    #[test]
    fn test_optimal_arrangement() {
        let seating_arrangement = SeatingArrangement::from_str(TEST_RULES).unwrap();
        assert_eq!(330, seating_arrangement.change_in_happiness(seating_arrangement.optimal_arrangement().as_slice()));
    }
}
