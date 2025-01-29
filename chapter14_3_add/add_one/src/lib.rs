//! Adds one to a given input value
//!
//!

/// Increment any number by 1.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        let result = add_one(3);
        assert_eq!(result, 4);
    }
}
