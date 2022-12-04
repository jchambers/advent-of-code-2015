use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let distances = Distances::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Total distance for shortest route: {}",
            distances.route_length(distances.optimal_route().as_slice())
        );

        println!(
            "Total distance for worst route: {}",
            distances.route_length(distances.worst_route().as_slice())
        );

        Ok(())
    } else {
        Err("Usage: day09 INPUT_FILE_PATH".into())
    }
}

struct Distances {
    distances: HashMap<String, HashMap<String, u32>>,
}

impl FromStr for Distances {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut distances = HashMap::new();

        for line in string.lines() {
            if let [destinations, distance] = line.split(" = ").collect::<Vec<&str>>().as_slice() {
                let distance = distance.parse()?;

                if let [a, b] = destinations.split(" to ").collect::<Vec<&str>>().as_slice() {
                    distances.entry(a.to_string()).or_insert_with(HashMap::new)
                        .insert(b.to_string(), distance);

                    distances.entry(b.to_string()).or_insert_with(HashMap::new)
                        .insert(a.to_string(), distance);
                } else {
                    return Err("Could not parse location pair".into());
                }
            } else {
                return Err("Could not parse entry".into());
            }
        }

        Ok(Distances { distances })
    }
}

impl Distances {
    fn locations(&self) -> impl Iterator<Item = &str> {
        self.distances.keys().map(|location| location.as_str())
    }

    fn optimal_route(&self) -> Vec<&str> {
        self.locations()
            .permutations(self.distances.len())
            .min_by_key(|route| self.route_length(route))
            .unwrap()
            .into_iter()
            .collect()
    }

    fn worst_route(&self) -> Vec<&str> {
        self.locations()
            .permutations(self.distances.len())
            .max_by_key(|route| self.route_length(route))
            .unwrap()
            .into_iter()
            .collect()
    }

    fn route_length(&self, route: &[&str]) -> u32 {
        route.windows(2)
            .map(|pair| self.distance(pair[0], pair[1]))
            .sum()
    }

    fn distance(&self, a: &str, b: &str) -> u32 {
        *self.distances.get(a).unwrap().get(b).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_LOCATIONS: &str = indoc!{"
        London to Dublin = 464
        London to Belfast = 518
        Dublin to Belfast = 141
    "};

    #[test]
    fn test_optimal_route() {
        let distances = Distances::from_str(TEST_LOCATIONS).unwrap();

        assert_eq!(605, distances.route_length(distances.optimal_route().as_slice()))
    }

    #[test]
    fn test_worst_route() {
        let distances = Distances::from_str(TEST_LOCATIONS).unwrap();

        assert_eq!(982, distances.route_length(distances.worst_route().as_slice()))
    }
}
