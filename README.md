# learnrust
Learning Rust


https://doc.rust-lang.org/book/ch00-00-introduction.html

Completed chapter 1:

Learned about Cargo

cargo new <PROJECT_ROOT>

Creates a new Rust project in PROJECT_ROOT (creates the directory PROJECT_ROOT if it does not exist)

cargo build and cargo build --release

cargo run for debugging

Completed chapter 2:

Completed the guessing game.

Learned about crates, Crates.io, dependencies .toml file.

Worked with references, data types, match, Ordering

Completed chapter 3:

Learned basic syntax around control flow, loops, conditionals, variable mutability, functions, and data types

Completed chapter 4:

Learned about references and borrowing. Use { } to manipulate scope if needed.
 - At any given time, you can have either one mutable reference or any number of immutable references.
 - References must always be valid.

 Completed chapter 5:

 Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

 Completed chapter 6:

 We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety: the compiler will make certain your functions only get values of the type each function expects.

In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules.

'if let' seems weird, but not super important.

Completed chapter 7:

Not a lot of code written, good to keep this chapter handy as a reference for organizing a real project once it's time.

Completed chapter 8:

Noted that slicing strings, did some cool stuff with hashmaps.

Completed chapter 9:

Of particular interest https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

Completed chapter 10:

Some stuff that will definitely need to be revisited regarding lifetime syntax and concepts.

Completed chapter 11:

Learned about 'cargo test -- --show-output'

Run single test 'cargo test NAME' or 'cargo test SUBNAME'

Ignore some tests with '#[ignore]'

Refer back to https://doc.rust-lang.org/book/ch11-03-test-organization.html for organization ideas.

Completed chapter 12, minigrep. Didn't implement env variables.

Completed chapter 15, will need more review, started chapter 16.