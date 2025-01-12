pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        panic!("Make this test fail");
    }

    #[test]
    fn other_asserts() {
        assert_eq!(2,2);
        assert_ne!(2,3);
        assert_eq!(3,4, "Custom message: 3 =/= 4");    // will fail
    }

    #[test]
    #[should_panic(expected = "array index out of bounds")] // doesn't really apply here
    fn using_should_panic() {
        let a = [1, 2, 3];
        let mut index = 0;
        loop {
            let x = a[index];
            index += 1;
        }
    }
}
