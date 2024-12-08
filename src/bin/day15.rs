use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::iter::Sum;
use std::ops::{Add, Mul};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let recipe = Recipe::from_str(fs::read_to_string(path)?.as_str())?;

        println!(
            "Optimal recipe score: {}",
            recipe.optimize_ingredients(None)
        );

        println!(
            "Optimal recipe score with 500 calorie target: {}",
            recipe.optimize_ingredients(Some(500))
        );

        Ok(())
    } else {
        Err("Usage: day15 INPUT_FILE_PATH".into())
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Recipe {
    ingredients: HashMap<String, Properties>,
}

impl Recipe {
    fn optimize_ingredients(&self, target_calories: Option<i32>) -> u64 {
        let properties: Vec<&Properties> = self.ingredients.values().collect();
        let mut quantities = vec![0; properties.len()];
        let mut stack: Vec<(usize, i32)> = (0..=100).map(|amount| (0, amount)).collect();

        let mut best_score = 0;

        while let Some((ingredient, amount)) = stack.pop() {
            quantities[ingredient] = amount;

            if ingredient == properties.len() - 1 {
                // We've reached the bottom of the "tree" and should evaluate
                let ingredients: Vec<(&Properties, i32)> = properties
                    .iter()
                    .cloned()
                    .zip(quantities.iter().cloned())
                    .collect();

                if let Some(target_calories) = target_calories {
                    if Self::calories(&ingredients) != target_calories {
                        continue;
                    }
                }

                best_score = max(best_score, Self::score(&ingredients));
            } else {
                // Keep exploring
                let used_teaspoons = quantities[0..=ingredient].iter().sum::<i32>();

                // If we're at the next-to-last level, we have a special consideration: the total
                // number of teaspoons in the recipe always has to add up to 100, so we should only
                // explore a final quantity that brings the recipe total up to 100
                if ingredient == properties.len() - 2 {
                    stack.push((ingredient + 1, 100 - used_teaspoons));
                } else {
                    stack.extend((0..=100 - used_teaspoons).map(|amount| (ingredient + 1, amount)));
                }
            }
        }

        best_score
    }

    fn score(ingredients: &[(&Properties, i32)]) -> u64 {
        debug_assert!(ingredients.iter().map(|(_, amount)| amount).sum::<i32>() == 100);

        let combined_properties: Properties = ingredients
            .iter()
            .map(|&(properties, quantity)| *properties * quantity)
            .sum();

        max(0, combined_properties.capacity) as u64
            * max(0, combined_properties.durability) as u64
            * max(0, combined_properties.flavor) as u64
            * max(0, combined_properties.texture) as u64
    }

    fn calories(ingredients: &[(&Properties, i32)]) -> i32 {
        debug_assert!(ingredients.iter().map(|(_, amount)| amount).sum::<i32>() == 100);

        let combined_properties: Properties = ingredients
            .iter()
            .map(|&(properties, quantity)| *properties * quantity)
            .sum();

        combined_properties.calories
    }
}

impl FromStr for Recipe {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ingredients = s
            .lines()
            .map(|line| {
                if let Some((name, properties)) = line.split_once(": ") {
                    Ok((
                        String::from(name),
                        Properties::from_str(properties)
                            .map_err(|_| "Could not parse properties")?,
                    ))
                } else {
                    Err("Could not parse ingredient line")
                }
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Recipe { ingredients })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Add for Properties {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Properties {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Mul<i32> for Properties {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Properties {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl Sum<Properties> for Properties {
    fn sum<I: Iterator<Item = Properties>>(iter: I) -> Self {
        let mut sum = Properties::default();

        for next in iter {
            sum = sum + next;
        }

        sum
    }
}

impl FromStr for Properties {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut capacity = None;
        let mut durability = None;
        let mut flavor = None;
        let mut texture = None;
        let mut calories = None;

        for pair in s.split(", ") {
            if let Some((property, value)) = pair.split_once(' ') {
                let value = value.parse::<i32>()?;

                match property {
                    "capacity" => capacity = Some(value),
                    "durability" => durability = Some(value),
                    "flavor" => flavor = Some(value),
                    "texture" => texture = Some(value),
                    "calories" => calories = Some(value),
                    _ => return Err("Unexpected property".into()),
                }
            } else {
                return Err("Could not parse property pair".into());
            }
        }

        if capacity.is_some()
            && durability.is_some()
            && flavor.is_some()
            && texture.is_some()
            && calories.is_some()
        {
            Ok(Properties {
                capacity: capacity.unwrap(),
                durability: durability.unwrap(),
                flavor: flavor.unwrap(),
                texture: texture.unwrap(),
                calories: calories.unwrap(),
            })
        } else {
            Err("Missing one or more properties".into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_RECIPE: &str = indoc! {"
        Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
        Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
    "};

    #[test]
    fn test_recipe_from_str() {
        let expected_recipe = Recipe {
            ingredients: HashMap::from([
                (
                    String::from("Butterscotch"),
                    Properties {
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8,
                    },
                ),
                (
                    String::from("Cinnamon"),
                    Properties {
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3,
                    },
                ),
            ]),
        };

        assert_eq!(expected_recipe, Recipe::from_str(TEST_RECIPE).unwrap());
    }

    #[test]
    fn test_recipe_optimize_ingredients() {
        assert_eq!(
            62842880,
            Recipe::from_str(TEST_RECIPE)
                .unwrap()
                .optimize_ingredients(None)
        );
    }

    #[test]
    fn test_recipe_optimize_ingredients_with_calorie_target() {
        assert_eq!(
            57600000,
            Recipe::from_str(TEST_RECIPE)
                .unwrap()
                .optimize_ingredients(Some(500))
        );
    }
}
