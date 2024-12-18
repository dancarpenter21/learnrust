// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    
    // scope of 's' begins here and this allocates on the stack, string literal
    let _s = "hello";

    heap_action();

    heap_action2();

    ownership_functions();

    references_borrowing();

    //let reference_to_nothing = dangle();  // fails to compile
    let _reference_to_nothing = dangle_correct();

    println!("Section 4.3 Slices");
    slices();
}   // predictably, scope for 's' ends here

fn heap_action() {
    let mut s = String::from("hi there");   // must declare 's' mutable to append
    s.push_str(", fella");
    // println!(s);     // this doesn't work, can't print non literal
    println!("{s}");
}

// Variables and Data Interacting with Move
fn heap_action2() {
    let x = 5;  // stack
    let y = x;  // stack, copied by value

    println!("{x},{y}");

    let s1 = String::from("hello"); // stack prt -> index 0, len 5, capacity 5, heap 'h' 'e' ...
    let s2 = s1;    // this is a copy by reference

    //println!("{s1}");   // memory safety makes this a compile error!
    println!("{s2}");

    // Scope and Assignment
    let mut s = String::from("hello");  // creates a compile warning, not unexpected
    s = String::from("ahoy");

    println!("{s}, world!");

    // Variables and Data Interacting with Clone
    let s3 = String::from("hello");
    let s4 = s3.clone();    // does a full heap copy

    println!("{s3}, {s4}");
}

fn ownership_functions() {
    let s = String::from("ownership and functions");

    takes_ownership(s);

    //println!("{s}");  // creates a compile error because 's' is taken by the function call

    let x = 78;

    makes_copy(x);

    println!("ownership_functions {x}");    // this is ok because the function doesn't take ownership of the i32

    let s: String = gives_ownership();
    println!("ownership_functions {s}");
}

fn takes_ownership(arg: String) {
    println!("takes_ownership {arg}");
}   // Here, arg goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(int: i32) {
    println!("makes_copy {int}");
}   // Here, int goes out of scope. Nothing special happens.

// Return Values and Scope
fn gives_ownership() -> String {
    // value comes into scope
    let value = String::from("return string");
    println!("gives_ownership returning {value}");

    // value is returned and moves out to the calling function
    value
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn references_borrowing() {
    let s1 = String::from("ref and borrow");
    let len = calc_length(&s1);
    println!("references_borrowing {s1} {len}");  // here i can use s1 still because of pass b reference in calc_length

    //change(&s1);  // cannot work because s1 is not mutable
    let mut s2 = String::from("borrow and change");
    change(&mut s2);
    println!("references_borrowing {s2}");

    let mut s4 = String::from("mutable");
    
    {
        let _r1 = &mut s4;
    }

    let _r2 = &mut s4;

    //println!("{r1}, {r2}"); // causes r2 to become a compile time error,
                            // the braces around r1 fix this error, but now r1 is out of scope

    let _r3 = &s4;  // no problem, non mutable borrow of a mutable thing
    let _r4 = &s4;  // also no problem, another immutable borrow
    let _r5 = &mut s4;  // BIG PROBLEM, cannot have a mutable borrow out when other immutable
                        // borrows are also active
    
    //println!("{_r3}, {_r4}, {_r5}");  // error actually gets thrown here

    let r6 = &s4;   // ok
    let r7 = &s4;   // also ok
    println!("{r6} and {r7}");  // this is ok so far

    let r8 = &mut s4;   // now this is ok because r6 and r7 are out of scope
    println!("r8 {r8}");
}

fn calc_length(text: &String) -> usize {
    text.len()
}

//fn change(text: &String) {    // cannot work because arg needs to be declared mutable as well
fn change(text: &mut String) {
    text.push_str(", updated");
}

/**
fn dangle() -> &String {
    // can't do this, s will go out of scope, so can't return a reference to it
    let s = String::from("dangling");
    &s
}
*/

fn dangle_correct() -> String {
    // have to do it like this, give back ownership
    let s = String::from("not dangling");
    s
}

// https://doc.rust-lang.org/book/ch04-03-slices.html
fn slices() {
    let cases = [
        String::from("onebigword"),
        String::from("just onespace"),
        String::from("at least a couple spaces")
    ];

    for case in &cases {
        let value = first_word(&case);
        println!("first word case {case} -> {value}");
    }

    for case in &cases {
        let value = second_word(&case);
        println!("second word case {case} -> {value}");
    }
}

// Here’s a small programming problem: write a function that takes a string of words
// separated by spaces and returns the first word it finds in that string.
// If the function doesn’t find a space in the string, the whole string must be one word,
// so the entire string should be returned.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut first_space = 0;
    let mut second_space = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if first_space == 0 {
                first_space = i;
            } else {
                second_space = i;
                break
            }
        }
    }

    return &s[first_space..second_space];
}