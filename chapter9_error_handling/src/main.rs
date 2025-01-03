use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};  // use std::io

// run with `RUST_BACKTRACE=1 cargo run`
fn main() {
    println!("Chapter 9 Error Handling");

    section9_1();
    section9_2();
}

// https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html
fn section9_1() {
    println!("Section 9.1: Unrecoverable Errors With 'panic!'");

    //panic!("crash and burn");

    // access array out of bounds
    {
        let v = vec![1, 2, 3];
        //v[99];
    }
}

fn section9_2() {
    println!("Section 9.2: Recoverable Errors with Result");
    
    // checking to see if files exist using result
    {
        let greeting_file_result = File::open("hello.txt");

        /*
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            
            // e.g. Os { code: 2, kind: NotFound, message: "No such file or directory" }
            Err(error) => panic!("Problem opening file: {error:?}"),
        };
        */
    }

    // try open a file and create it if it doesn't exist
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_fie = match greeting_file_result {
            Ok(file) => file,

            // file doesn't exist
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Could not create the file: {e:?}"),
                },

                // e.g. Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
                // if you remove write permissions from the directory
                other_error => panic!("Problem opening file: {other_error:?}"),
            },
        };
    }

    // Shortcuts for Panic on Error: unwrap and expect
    {
        // Instead of using all these nested matches like this:
        /*
        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            
            // e.g. Os { code: 2, kind: NotFound, message: "No such file or directory" }
            Err(error) => panic!("Problem opening file: {error:?}"),
        };
        */

        //we can:

        let greeting_file = File::open("hello.txt")
            .unwrap();
        // or we can use 'expect' (preferred over 'unwrap')
        let greeting_file = File::open("hello.txt")
            .expect("hello.txt should be included in the project");
    }

    // propagating errors example
    {
        let content_result = read_from_file();

        let content = match content_result {
            Ok(text) => text,
            Err(e) => panic!("Could not read content: {e:?}"),
        };

        println!("{content}");
    }
}

// Propagating Errors
fn read_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }   // no ';' here because we're returning

}