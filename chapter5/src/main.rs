struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self /*: &Self */) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated functions, often constructors
    // note that square does not have a &self parameter
    // this is like static methods
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// tuple structs
struct Color(u8, u8, u8);
struct Point(f32, f32, f32);

// Unit-Like struct
struct AlwaysEqual;

fn new_user(email: String, username: String) -> User {
    User {
        active: true,
        //username: username,
        username,   // cool shorthand!
        email,
        sign_in_count: 1,
    }
}

fn main() {
    section5_1();

    section5_2();

    section5_3();
}

// https://doc.rust-lang.org/book/ch05-01-defining-structs.html
fn section5_1() {
    println!("Section 5.1 Defining and Instantiating Structs");

    let user1 = User {
        active: true,
        username: String::from("xxhotshotxx"),
        email: String::from("rustacean007@rust.org"),
        sign_in_count: 1,
    };

    /*
    // lifetime error if you try to use references in a struct, chapter 10 addresses
    let user1 = User {
        active: true,
        username: "xxhotshotxx",        // &str slice is invalid here
        email: "rustacean007@rust.org", // &str slice is invalid here
        sign_in_count: 1,
    };
    */


    // have to declare item mutable to change field values
    let mut user2 = new_user(String::from("nooblet@rust.org"), String::from("boringguy"));
    user2.email = String::from("realbignoob@gmail.com");

    let user3 = User {
        username: String::from("secretnoob"),
        ..user2
    };

    println!("{}", user2.username);
    println!("{}", user3.username);
    //println!("{}", user2.email);    // user3 took ownership here because of a move, but only of that single field!
    println!("{}", user3.email);

    let _black = Color(0,0,0);          // black
    let _origin = Point(0.0,0.0,0.0);   // origin

    let _subject = AlwaysEqual;
}

// https://doc.rust-lang.org/book/ch05-02-example-structs.html
fn section5_2() {
    println!("Section 5.2 Example Program With Structs");
    with_values();
    with_tuples();
    with_struct();

    let rect1 = Rectangle {
        width: 45,
        height: 56,
    };

    //println!("Rectangle {}", rect);   // won't work until we implement std::format::Display
    println!("Rectangle {:?}", rect1);   // this won't work until we implement Debug trait
    println!("Rectangle {:#?}", rect1);   // prett print debug

    // Another way to print out a value using the Debug format is to use the dbg! macro,
    // which takes ownership of an expression (as opposed to println!, which takes
    // a reference), prints the file and line number of where that dbg! macro call
    // occurs in your code along with the resultant value of that expression, and
    // returns ownership of the value.
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

fn with_values() {
    let w1 = 30;
    let h1 = 50;
    println!(
        "The area of the rectangle is {}",
        area(w1, h1)
    );
}

fn with_tuples() {
    let rect = (40, 60);
    println!(
        "The area of the rectangle is {}",
        area_tuple(rect)
    );
}

fn with_struct() {
    let rect = Rectangle {
        width: 15,
        height: 60,
    };

    println!(
        "The area of the rectangle is {}",
        area_rect(&rect)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn section5_3() {
    println!("Section 5.3 Method Syntax");
    let rect1 = Rectangle {
        width: 12,
        height: 36,
    };

    println!("area {}", rect1.area());

    let square1 = Rectangle::square(55);
    println!("area {}", square1.area());
}