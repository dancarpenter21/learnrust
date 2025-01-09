use std::time::SystemTime;

// finds first N prime numbers, performane comparison of Vec vs Array
fn main() {
    println!("Prime number search is O(n log n)");

    const COUNT: usize = 3_000_000;

    // do with Vec
    {
        let mut primes = vec![];

        let start_time = SystemTime::now();

        let mut number = 0;
        while primes.len() < COUNT {
            let number_is_prime: bool = is_prime_vec(number, &primes);
            if number_is_prime {
                primes.push(number);
            }

            number += 1;
        }

        match start_time.elapsed() {
            Ok(elapsed) => println!("Time elapsed (vec) {}", elapsed.as_millis()),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Last prime (vec): {}", primes[COUNT-1]);
    }

    // do with arrays
    {
        let mut primes: [i32; COUNT] = [0; COUNT];
        let start_time = SystemTime::now();

        let mut number = 0;
        let mut current_index = 0;
        loop {
            let number_is_prime: bool = is_prime_array(number, &primes, current_index);
            if number_is_prime {
                primes[current_index] = number;
                current_index += 1;
            }

            if current_index == COUNT {
                break;
            }

            number += 1;
        }

        match start_time.elapsed() {
            Ok(elapsed) => println!("Time elapsed (array) {}", elapsed.as_millis()),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Last prime (array): {}", primes[COUNT-1]);
    }

}

fn is_prime_vec(number: i32, primes: &Vec<i32>) -> bool {
    let sqrt_num = (number as f32).sqrt().ceil() as i32;
    if number < 2 {
        return false;
    }

    if number == 2 || number == 3 {
        return true;
    }

    for prime in primes {

        if *prime > sqrt_num {
            return true;
        }

        if number % *prime == 0 {
            // not a prime number
            return false;
        }
    }

    // we ran off the end of our list and didn't find a
    // 0 modulo, so prime
    return true;
}

fn is_prime_array(number: i32, primes: &[i32], current_count: usize) -> bool {
    let sqrt_num = (number as f32).sqrt().ceil() as i32;
    let mut prime_index = 0;

    if number < 2 {
        return false;
    }

    if number == 2 || number == 3 {
        return true;
    }

    loop {
        let prime = primes[prime_index];

        if prime > sqrt_num {
            return true;
        }

        if number % prime == 0 {
            // not a prime number
            return false;
        }

        prime_index += 1;
        if prime_index == current_count {
            // not a prime number, we ran off the end of our list
            // this is not likely to happen unless there is a vast span
            // of no prime numbers
            return true;
        }
    }
}