use std::any;

// 'const' variables cannot be computed, they must be true constants
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    println!("Chapter 3 of learning rust book");

    section3_1();

    section3_2();

    section3_3(6, 4.5, "string of text");

    section3_5();
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
fn section3_5() {
    println!("Section 3.5 Control Flow");
    let num = 5;
    if num < 7 {
        println!("Condition true");
    } else {
        println!("Condition false");
    }

    let condition = true;
    let value = if condition {6} else {10};
    println!("Conditional set is {value}");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Counter {result}");

    // loops and loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    // while loop
    while false {
        println!("this will never happen");
    }

    let array = [1, 3, 4, 6, 7, 8, 10];
    for i in array {
        println!("{i}");
    }

    // print 98, 97, ... 2, 1
    for number in (1..99).rev() {
        println!("{number}");
    }
}

// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
fn section3_3(i: i32, f: f64, text: &str) {
    println!("Section 3.3 Expressions");

    println!("i {i} {} f {f} {} text {text} {}",
             get_type(&i), get_type(&f), get_type(&text));

    // Statements are instructions that perform some action and do not return a value. Function definitions are statements.
    // Expressions evaluate to a resultant value. Let’s look at some examples.
    let _x = 5;
    //let x = let y = 6   // this won't work because 'y=6' as a statement does not actually return anything
    //let x = let y = 6;  // this expression is not valid!

    let y = {
        let x = 3;
        x + 1
    };

    println!("y {y}");
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html
fn section3_2() {
    println!("Section 3.2 Data Types");

    // ints
    let big_int = 98_222;   // 98,222
    println!("big_int {big_int} {}", get_type(&big_int));
    let hex = 0xFA;
    println!("hex {hex} {}", get_type(&hex));
    let octal = 0o76;
    println!("octal {octal} {}", get_type(&octal));
    let binary = 0b1111_0101;
    println!("binary {binary} {}", get_type(&binary));
    let byte: u8 = b'B';
    println!("byte {byte} {}", get_type(&byte));

    // floats
    let float64 = 3.14;
    println!("float64 {float64} {}", get_type(&float64));
    let float32: f32 = 2.78;
    println!("float32 {float32} {}", get_type(&float32));

    // operations
    let _sum = 3 + 7;
    let _difference = 457.23 - 45.0; // cannot infer types here
    let _product = 45*78;
    let _quotient = 56.7 / 346.2;
    let _truncated = -5 / 3; // results in -1
    let _remainder = 43 % 5;

    // booleans
    let _tru = true;
    let _fals: bool = false;

    // char
    let _c = 'z';
    let _z: char = 'Z';
    let heart_cat = '😻';
    println!("heart_cat {heart_cat} {}", get_type(&heart_cat));

    // tuples
    let tup: (i32, f64, u8) = (34_567, 3.1415, 129);
    println!("tup ({} {} {}) ({} {} {})", tup.0, tup.1, tup.2,
        get_type(&tup.0), get_type(&tup.1), get_type(&tup.2));

    // arrays
    let _a = [1, 3, 5, 6, 7];
    let _a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let a = [0; 5];     // [0, 0, 0, 0, 0]
    println!("a0 {}", a[0]);
}

// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
fn section3_1() {
    println!("Section 3.1 Variables and Mutability");

    // ===================================================
    let mut x = 5;  // assign value of 5 to x
    println!("x = {x}");

    // will raise a compile time error if 'x' is not declared with 'mut'
    x = 6;
    println!("x = {x}");

    println!("Three hours in seconds {THREE_HOURS_IN_SECONDS}");

    // =====================================================
    // shadowing variable 'x', also note how can redeine 'x'
    let x = 10;

    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner x = {x}");
    }

    println!("Outer x = {x}");

    // =====================================================
    // shadowing continued
    let spaces = "         ";       // spaces starts as a string
    let spaces = spaces.len();  // spaces is now an i32 or u32
    println!("spaces = {spaces}");
}

// https://stackoverflow.com/a/58119924
fn get_type<T>(_: &T) -> String {
    return any::type_name::<T>().to_string();
}
