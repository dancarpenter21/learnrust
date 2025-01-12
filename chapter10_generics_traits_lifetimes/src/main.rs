use std::fmt::Display;  // used in summary fuction

fn main() {
    println!("Chapter 10: Generic Types, Traits, and Lifetimes");

    section10_1();
    // section 10.2 was mostly reading and looking at syntax for Traits
    // note this syntax for passing Traits as parameters
    /*
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    */

    // 10.2 also discusses some other nice examples when diving deep into generics

    // fn returns_summarizable() -> impl Summary { ... } to return a Trait

    // https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
    // is particularly interesting in 10.2 for implementing to_string on custom types

    section10_3();

    chapter10_summary();
}

// https://doc.rust-lang.org/book/ch10-01-syntax.html
fn section10_1() {
    println!("Section 10.1 Generic Data Types");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let number_list = [300, 24, 3, -5, 50];
    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}

// note that this works with Vec and Array, because &[i32] is
// a slice of numbers
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// replace all three helpers with this generic version
// we take a slice of T's instead of a concrete type
// The help text mentions std::cmp::PartialOrd, which is a trait,
// and we’re going to talk about traits in the next section. For now,
// know that this error states that the body of largest won’t work for
// all possible types that T could be. Because we want to compare
// values of type T in the body, we can only use types whose values can
// be ordered. To enable comparisons, the standard library has the
// std::cmp::PartialOrd trait that you can implement on types
// (see Appendix C for more on this trait).
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        /*
        if item > largest { // will cause an error
            largest = item;
        }
        */
    }

    largest
}

/* Example of multiple generic
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
*/

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn section10_3() {
    println!("Section 10.3 Validating References with Lifetimes");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let novel = String::from("Call me Dan. This is my story ...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

/*
The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can’t tell whether the reference being returned refers to x or y.
Actually, we don’t know either, because the if block in the body of this function
returns a reference to x and the else block returns a reference to y!
Adding 'a to the parameters and function definition fix this issue.
*/
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// if we want our structs to contain references ...
struct ImportantExerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // here are two input lifetimes, so Rust applies the first lifetime
    // elision rule and gives both &self and announcement their own lifetimes.
    // Then, because one of the parameters is &self, the return type gets the
    // lifetime of &self, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn chapter10_summary() {
    println!("Chapter 10 Tying It All Together");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let novel = String::from("Call me Dan. This is my story ...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExerpt {
        part: first_sentence,
    };

    let result = longest_with_announcement(string1.as_str(), string2, novel);
    println!("The longest string is {result}");

}

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
