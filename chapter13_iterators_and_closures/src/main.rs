
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Yellow,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // If the Option<T> is the Some variant, unwrap_or_else
        // returns the value from within the Some. If the Option<T>
        // is the None variant, unwrap_or_else calls the closure and
        // returns the value returned by the closure.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_yell = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Yellow => num_yell += 1,
            }
        }

        if num_red > num_blue {
            if num_red > num_yell {
                ShirtColor::Red
            } else {
                ShirtColor::Yellow
            }
        } else {
            if num_blue > num_yell {
                ShirtColor::Blue
            } else {
                ShirtColor::Yellow
            }
        }
    }
}

fn main() {
    println!("Chapter 13: Iterators and Closures");
    section13_1();
    section13_2();

}

fn section13_1() {
    println!("Section 13.1 Closures");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Yellow, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn section13_2() {
    println!("Section 13.2 Series Of Items w/ Iterators");

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {val}");
    }
}
