

// 'const' variables cannot be computed, they must be true constants
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    println!("Chapter 3 of learning rust book");

    
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

    let mut spaces = "       ";
    //spaces = spaces.len();  // not allowed change type like this
    let spaces = spaces.len();  // this is ok with 'let' but warning
    println!("spaces = {spaces}");

}
