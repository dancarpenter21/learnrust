use std::collections::HashMap;

fn main() {
    println!("Chapter 8 Collections");

    section8_1();

    section8_2();

    section8_3();
}

fn section8_1() {
    println!("Section 8.1 Storing Lists With Vectors");

    {
        let mut v: Vec<i32> = Vec::new();   // let v = vec![1, 2, 3];   is also valid with the vec! macro

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        v.push(9);

        for i in v.iter() {
            println!("{i}");
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        let third = &v[2];
        println!("Third element of v is {third}");

        // safe access
        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("Third element of v is {third}"),
            None => println!("No third element"),
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];
        let first = v[0];
        v.push(6);
        println!("The first item in v is {first}");
        for i in v.iter() {
            println!("{i}");
        }
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {   // the '&' is necessary otherwise the 'v' gets taken
            println!("{i}");
        }

        let first = v[0];
        println!("First item is {first}");
    }

    // When the vector gets dropped, all of its contents are also dropped,
    // meaning the integers it holds will be cleaned up. The borrow checker
    // ensures that any references to contents of a vector are only used while
    // the vector itself is valid.
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;   // dereference the mutable borrow ref and add 50 in place
        }

        for i in v {    // note we eat 'v' here so it couldn't be used later in this scope
            println!("{i}");
        }
    }
}

fn section8_2() {
    println!("Section 8.2 UTF-8 Text Strings");

    {
        // equivalent methods
        //let s = String::new();
        //let s = String::from("initial content");
        let s = "initial content".to_string();
        println!("{s}");
    }

    // appending to strings
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);    // this probably works because s2 is a &str so it doesn't get eaten
        println!("s1 is {s1}");
        println!("s2 is {s2}");
    }

    {
        let mut s = "lo".to_string();
        s.push('l');    // append a single char
        println!("{s}");
    }

    // string concatenation with '+'
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("plus sign");
        let s3 = s1 + &s2; // string concatenation requires an owned `String` on the left
                            // help: remove the borrow to obtain an owned `String`
                            // i.e. s1 has been moved and is no longer valid
        //println!("{s1}");
        println!("{s2}");
        println!("{s3}");
    }

    // format macro for unwieldy operations
    {
        let s1 = String::from("tic");
        let s2 = "tac".to_string();
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");  // returns String or &String and doesn't
                                            // take ownership of any parameters
        println!("{s}");
    }
}

// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
fn section8_3() {
    println!("Section 8.3 Keys And Hashmaps");

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Here, score will have the value that’s associated with the Blue team, and the
        // result will be 10. The get method returns an Option<&V>; if there’s no value
        // for that key in the hash map, get will return None.
        // This program handles the Option by calling copied to get an Option<i32> rather
        // than an Option<&i32>, then unwrap_or to set score to zero if scores doesn’t
        // have an entry for the key.
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        println!("{score}");

        // iterate over map
        // don't forget '&scores' so that the loop doesn't take ownership of our map
        for (key, value) in &scores {   
            println!("{key}: {value}");
        }
    }

    // For types that implement the Copy trait, like i32, the values are copied into the
    // hash map. For owned values like String, the values will be moved and the hash map
    // will be the owner of those values, as demonstrated in Listing 8-22.
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        //println!("{field_name}: {field_value}");    // value move / borrow error here
    }

    // Adding a Key and Value Only If a Key Isn’t Present
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // Using the entry method to only insert if the key does not already have a value.
        // 'entry()' returns an Enum type.
        scores.entry(String::from("Yellow")).or_insert(50); 
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}");
    }

    // Updating a value based on an old value
    {
        let text = String::from("hello world wonderful world");
        let mut map = HashMap::new();

        // this is slick!
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{map:?}");
    }
}