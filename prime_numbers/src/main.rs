use std::time::SystemTime;

pub mod prime_finder;
use crate::prime_finder::PrimeFinder;

fn main() {
    const LIMIT: i64 = 2_000_000;

    let slow_primes = Vec::new();
    let faster_primes = Vec::new();
    let fastest_primes = Vec::new();

    {
        println!("Find Prime Numbers in the slowest possible way ...");
        let slow_time = SystemTime::now();
        let mut i: i64 = 1;
        let mut count = 0;
        loop {
            if is_prime_slow(i) {
                count += 1;
                //slow_primes.push(i);
                //println!("{i}");
            }

            i += 1;

            if i == LIMIT {
                break;
            }
        }

        match slow_time.elapsed() {
            Ok(elapsed) => println!("Slow time {} {}", elapsed.as_millis(), count),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Slow one done!");
    }

    {
        println!("Find Prime Numbers in a slightly faster way ...");
        let faster_time = SystemTime::now();
        let mut i: i64 = 1;
        let mut count = 0;
        loop {
            if is_prime_fast(i) {
                count += 1;
                //faster_primes.push(i);
                //println!("{i}");
            }

            i += 1;

            if i == LIMIT {
                break;
            }
        }

        match faster_time.elapsed() {
            Ok(elapsed) => println!("Faster time {} {}", elapsed.as_millis(), count),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Faster one done!");
    }

    {
        println!("Find Prime Numbers in the fastest way ...");
        let fastest_time = SystemTime::now();
        let mut i: i64 = 1;
        let mut prime_list = PrimeFinder::new();
        
        let mut count = 0;
        loop {
            if prime_list.is_prime(i) {
                count += 1;
                //fastest_primes.push(i);
                //println!("The {count} prime is {i}");
            }

            i += 1;

            if i == LIMIT {
                break;
            }
        }

        match fastest_time.elapsed() {
            Ok(elapsed) => println!("Fastest time {} {}", elapsed.as_millis(), count),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Fastest one done!");
    }

    {
        let mut print_index: usize = 0;
        loop {
            if print_index == LIMIT.try_into().unwrap() {
                break;
            }

            let slow_opt: Option<&i64> = slow_primes.get(print_index);
            let faster_opt: Option<&i64> = faster_primes.get(print_index);
            let fastest_opt: Option<&i64> = fastest_primes.get(print_index);

            
            let slow_prime = match slow_opt {
                Some(slow) => *slow,
                None => -1,
            };

            let faster_prime = match faster_opt {
                Some(faster) => *faster,
                None => -1,
            };

            let fastest_prime = match fastest_opt {
                Some(fastest) => *fastest,
                None => -1,
            };

            println!("Number {}: {slow_prime} {faster_prime} {fastest_prime}", print_index+1);

            if slow_prime == -1 || faster_prime == -1 || fastest_prime == -1 {
                break;
            }
            
            print_index += 1;
        }
    }
}

fn is_prime_fast(i: i64) -> bool {
    if i < 2 {
        return false;
    }

    if i == 2 {
        return true;
    }

    if i % 2 == 0 {
        return false;
    }

    if i == 3 {
        return true;
    }
    
    let mut j = 3;
    let max = (i as f64).sqrt().ceil() as i64;
    loop {
        if max < j {
            return true;
        }

        if i % j == 0 {
            return false;
        }

        j += 2;
    }
}

fn is_prime_slow(i: i64) -> bool {
    if i < 2 {
        return false;
    }

    if i == 2 {
        return true;
    }

    if i % 2 == 0 {
        return false;
    }

    if i == 3 {
        return true;
    }

    let mut j = 3;
    loop {
        if i == j {
            return true;
        }

        if i % j == 0 {
            return false;
        }

        j += 2;
    }
}