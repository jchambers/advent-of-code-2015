use std::error::Error;
use std::ops::AddAssign;
use std::str::FromStr;
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(initial_password) = args.get(1) {
        let password = Password::from_str(initial_password)?;

        println!(
            "Next valid password after {}: {}",
            initial_password,
            password.next_valid_password().to_string()
        );

        println!(
            "Next valid password after {}: {}",
            password.next_valid_password().to_string(),
            password.next_valid_password().next_valid_password().to_string()
        );

        Ok(())
    } else {
        Err("Usage: day11 INITIAL_PASSWORD".into())
    }
}

const ALPHABET: [char; 23] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z'
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Password {
    password: u64,
    min_string_length: usize,
}

impl FromStr for Password {
    type Err = Box<dyn Error>;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut password = 0;

        for c in string.chars() {
            let char_val = match c {
                'a'..='h' => c as u64 - 'a' as u64,
                'j' | 'k' => c as u64 - 'a' as u64 - 1,
                'm' | 'n' => c as u64 - 'a' as u64 - 2,
                'p'..='z' => c as u64 - 'a' as u64 - 3,
                _ => return Err("Unexpected character".into()),
            };

            password = (password * ALPHABET.len() as u64) + char_val;
        }

        Ok(Password { password, min_string_length: string.chars().count() })
    }
}

impl ToString for Password {
    fn to_string(&self) -> String {

        let mut reverse = String::new();
        let mut password = self.password;

        loop {
            reverse.push(ALPHABET[password as usize % ALPHABET.len()]);
            password /= ALPHABET.len() as u64;

            if password == 0 {
                break;
            }
        }

        // We may need to pad the string representation with leading zeroes
        while reverse.chars().count() < self.min_string_length {
            reverse.push(ALPHABET[0]);
        }

        reverse.chars().rev().collect()
    }
}

impl AddAssign<u64> for Password {
    fn add_assign(&mut self, rhs: u64) {
        self.password += rhs;
    }
}

impl Password {

    fn next_valid_password(&self) -> Self {
        let mut next = *self;

        loop {
            next += 1;

            if next.is_valid() {
                return next;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let password_str = self.to_string();

        // Because our alphabet excludes illegal characters, we can assume passwords never contain
        // illegal characters and can skip to the other two checks.
        Self::contains_increasing_straight(password_str.as_str()) &&
            Self::has_non_overlapping_repeated_pairs(password_str.as_str())
    }

    fn contains_increasing_straight(password_str: &str) -> bool {
        password_str.chars()
            .tuple_windows()
            .any(|(a, b, c)| b as u32 == a as u32 + 1 && c as u32 == b as u32 + 1)
    }

    fn has_non_overlapping_repeated_pairs(password_str: &str) -> bool {
        let mut remainder = password_str;
        let mut repeated_pairs = 0;

        while remainder.len() >= 2 {
            if remainder[0..1] == remainder[1..2] {
                repeated_pairs += 1;
                remainder = &remainder[2..];
            } else {
                remainder = &remainder[1..];
            }

            if repeated_pairs >= 2 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_from_string() {
        let password_string = "xz";
        assert_eq!(password_string, Password::from_str(password_string).unwrap().to_string());
    }

    #[test]
    fn test_add_assign() {
        {
            let mut password = Password::from_str("xz").unwrap();
            password += 1;

            assert_eq!("ya", password.to_string());
        }

        {
            let mut password = Password::from_str("abcdffaa").unwrap();
            password += 1;

            assert_eq!("abcdffab", password.to_string());
        }
    }

    #[test]
    fn password_contains_increasing_straight() {
        assert!(Password::contains_increasing_straight("hijklmmn"));
        assert!(!Password::contains_increasing_straight("abbceffg"));

        assert!(Password::contains_increasing_straight("abcdffaa"));
    }

    #[test]
    fn password_has_non_overlapping_repeated_pairs() {
        assert!(Password::has_non_overlapping_repeated_pairs("abbceffg"));
        assert!(!Password::has_non_overlapping_repeated_pairs("abbcegjk"));

        assert!(Password::has_non_overlapping_repeated_pairs("abcdffaa"));
    }

    #[test]
    fn test_is_valid_password() {
        assert!(Password::from_str("abcdffaa").unwrap().is_valid());
        assert!(Password::from_str("ghjaabcc").unwrap().is_valid());
    }

    #[test]
    fn next_valid_password() {
        assert_eq!(
            Password::from_str("abcdffaa").unwrap(),
            Password::from_str("abcdefgh").unwrap().next_valid_password()
        );
    }
}