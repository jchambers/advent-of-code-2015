use std::sync::{mpsc, Arc, Mutex};
use std::{env, error, thread};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(key) = args.get(1) {
        println!(
            "First AdventCoin for {} with 5 leading zeroes: {}",
            key,
            find_advent_coin(key, 5, num_cpus::get())
        );
        println!(
            "First AdventCoin for {} with 6 leading zeroes: {}",
            key,
            find_advent_coin(key, 6, num_cpus::get())
        );
        Ok(())
    } else {
        Err("Usage: day04 KEY".into())
    }
}

fn find_advent_coin(key: &str, leading_zeroes: usize, threads: usize) -> u32 {
    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();
    let counter = Arc::new(Mutex::new(0));
    let found_advent_coin = Arc::new(Mutex::new(false));

    for _ in 0..threads {
        let counter = counter.clone();
        let tx = tx.clone();
        let found_advent_coin = found_advent_coin.clone();
        let key = String::from(key);
        let prefix = "0".repeat(leading_zeroes);

        handles.push(thread::spawn(move || loop {
            let done = {
                let done = found_advent_coin.lock().unwrap();
                *done
            };

            if done {
                break;
            }

            let n = {
                let mut n = counter.lock().unwrap();
                *n += 1;

                *n
            };

            let digest = md5::compute(format!("{}{}", key, n).as_bytes());

            if format!("{:x}", digest).starts_with(&prefix) {
                match tx.send(n) {
                    Ok(_) => {
                        let mut found_coin = found_advent_coin.lock().unwrap();
                        *found_coin = true;
                    }
                    Err(_) => break,
                }
            }
        }));
    }

    let first_coin = rx.recv().unwrap();
    drop(rx);

    for handle in handles {
        handle.join().unwrap();
    }

    first_coin
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_advent_coin() {
        assert_eq!(609043, find_advent_coin("abcdef", 5, num_cpus::get()));
        assert_eq!(1048970, find_advent_coin("pqrstuv", 5, num_cpus::get()));
    }
}
