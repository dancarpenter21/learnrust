struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
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
}