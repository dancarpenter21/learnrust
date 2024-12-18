// basic enum
enum IpAddressKind {
    V4,
    V6,
}

// typed enum
enum IpAddressType {
    V4(String),
    V6(String),
}

// more flexible
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                           // has no data associated
    Move { x: i32, y: i32 },        // has two named fields like a struct
    Write(String),                  // has a single string
    ChangeColor(i32, i32, i32),     // as 3 i32 for RGB
}

impl Message {
    fn call(&self) {
        // can define methods on enum types
        println!("Called message");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Delaware,
    Pennsylvania,
    Virginia,
    New_York,
    Massachussets,
    Maryland,
}

fn main() {
    println!("Chapter 6 Enums and Pattern Matching");

    section6_1();

    section6_2();

    section6_3();
}

fn section6_1() {
    println!("Section 6.1 Defining an Enum");
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let home = IpAddressType::V4(String::from("127.0.0.1"));
    let loopback = IpAddressType::V6(String::from("::1"));

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hi there"));
    m.call();
}

fn route(ip_kind: IpAddressKind) { }

fn section6_2() {
    println!("Section 6.2 Control Flow - match");
    
    let value = value_in_cents(Coin::Penny);
    println!("{value}");

    let value = value_in_cents(Coin::Quarter(UsState::New_York));
    println!("{value}");

    let five = Some(5);
    let six = plus_one(five);
    println!("{}", six.unwrap());
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {   // add the 'state' variable to build to the UsState held here
            println!("State quarter of {state:?}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => x,
        Some(i) => {
            Some(i+1)
        },
    }
}

fn section6_3() {
    // this is not assignment despite the '=' sign
    println!("Section 6.3 Concise Flow with 'let if'");

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is {max}");
    }
}
