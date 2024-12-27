use std::time::SystemTime;

fn main() {

    {
        println!("Find Prime Numbers in the slowest possible way ...");
        let slow_time = SystemTime::now();
        let mut i: i32 = 1;
        loop {
            if is_prime_slow(&i) {
                //println!("{i}");
            }

            i += 1;

            if i == 1_000_000 {
                break;
            }
        }

        match slow_time.elapsed() {
            Ok(elapsed) => println!("Slow time {}", elapsed.as_millis()),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Slow one done!");
    }

    {
        println!("Find Prime Numbers in a slightly faster way ...");
        let faster_time = SystemTime::now();
        let mut i: i32 = 1;
        loop {
            if is_prime_fast(&i) {
                //println!("{i}");
            }

            i += 1;

            if i == 1_000_000 {
                break;
            }
        }

        match faster_time.elapsed() {
            Ok(elapsed) => println!("Faster time {}", elapsed.as_millis()),
            Err(e) => println!("Error: {:?}", e),
        }

        println!("Faster one done!");
    }
}

fn is_prime_fast(i: &i32) -> bool {
    if *i < 2 {
        return false;
    }

    if *i == 2 {
        return true;
    }

    if *i % 2 == 0 {
        return false;
    }

    if *i == 3 {
        return true;
    }
    
    if *i == 5 {
        return true;
    }

    let mut j: i32 = 7;
    let max = (*i as f64).sqrt().ceil() as i32;
    loop {
        if max <= j {
            return true;
        }

        if *i % j == 0 {
            return false;
        }

        j += 2;
    }
}

fn is_prime_slow(i: &i32) -> bool {
    if *i < 2 {
        return false;
    }

    if *i == 2 {
        return true;
    }

    if *i % 2 == 0 {
        return false;
    }

    if *i == 3 {
        return true;
    }
    
    if *i == 5 {
        return true;
    }

    let mut j: i32 = 5;
    loop {
        if *i <= j {
            return true;
        }

        if *i % j == 0 {
            return false;
        }

        j += 2;
    }
}