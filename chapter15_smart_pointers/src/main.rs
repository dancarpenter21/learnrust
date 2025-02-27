
use crate::List::{ Cons, Nil };     // used in 15.1
use std::ops::Deref;                // used in 15.2
//use std::rc::Rc;

fn main() {
    println!("Chapter 15: Smart Pointers");
    section15_1();
    section15_2();
    section15_3();
    section15_4();
}

fn section15_1() {
    println!("Section 15.1 Using Box<T> For Data on the Heap");

    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn section15_2() {
    println!("Section 15.2 Deref Trait");

    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // you can use Box<T> like a reference, because the
    // Box trait implements '*'
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // MyBox won't work like this until it
    // implements '*'
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    // Rust has deref coercion
    {
        let m = MyBox::new(String::from("Rustacean"));
        hello(&m);  // forgetting the '&' here gives vague error message
        hello(&(*m)[..]);   // w/o deref coercion
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }

}

// allows us to dereference with '*'
impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for
    // the Deref trait to use
    type Target = T;    

    fn deref(&self) -> &Self::Target {
        &self.0     // note the '&' here!
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn section15_3() {
    println!("Section 15.3 Drop Trait");
    let c = CustomSmartPointer {
        data: String::from("the C"),
    };

    let d = CustomSmartPointer {
        data: String::from("the D"),
    };

    let e = CustomSmartPointer {
        data: String::from("the E"),
    };

    drop(e);

    println!("CSP created");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CSP {}", &self.data);
    }
}

fn section15_4() {
    println!("Section 15.4 Ref Counted Smart Pointer");
    println!("Skipped this lesson for the time being");
    //let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //let b = Cons(3, Rc::clone(&a));
    //let c = Cons(4, Rc::clone(&a));
}

fn section15_5() {
    println!("Section 15.5 Interior Mutability");

    //let x = 5;
    //let y = &mut x;
}