
#[derive(Debug)]
pub struct PrimeFinder {
    found_set: Vec<i64>,
}

impl PrimeFinder {
    pub fn new() -> Self {
        Self {
            found_set: vec![2, 3, /*5, 7, 11, 13, 17, 19, 23,
                            29, 31, 37, 41, 43, 47, 53, 59,
                            61, 67, 71, 73, 79, 83, 89, 97*/],
        }
    }

    pub fn get_count(&self) -> usize {
        return self.found_set.len();
    }

    // this function uses the fundamental theorem of arithmetic
    pub fn is_prime(&mut self, number: i64) -> bool {

        // check for domain
        if number < 2 {
            return false;
        }

        let last_index: usize = self.found_set.len() - 1;
        let last_prime = self.found_set[last_index];

        if number == last_prime {
            return true;
        } else if number < last_prime {
            return self.check_in_set(number);
        } else {
            return self.check_new(number);
        }
    }

    fn check_new(&mut self, number: i64) -> bool {
        let last_index = self.found_set.len() - 1;
        let last_prime = self.found_set[last_index];

        let sqrt_number = (number as f64).sqrt().ceil() as i64;

        if sqrt_number <= last_prime {
            for i in self.found_set.iter_mut() {
                let next_prime = *i;
                if next_prime > sqrt_number {
                    break;
                }

                if number % next_prime == 0 {
                    return false;
                }
            }

            return true;

        } else {
            self.grow_to(sqrt_number);
            return self.check_new(number);
        }
    }

    fn grow_to(&mut self, sqrt_num: i64) {
        let last_index = self.found_set.len() - 1;
        let last_prime = self.found_set[last_index];

        let mut try_this = last_prime + 2;
        loop {
            let is_prime = self.check_new(try_this);
            if is_prime {
                self.found_set.push(try_this);
                
                if try_this > sqrt_num {
                    break;
                }
            }
            
            try_this += 2;
        }
    }

    fn check_in_set(&mut self, number: i64) -> bool {
        for i in self.found_set.iter_mut() {
            if number == *i {
                return true;
            }
        }

        return false;
    }
}