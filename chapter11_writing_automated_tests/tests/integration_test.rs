use chapter11_writing_automated_tests::add;

#[test]
fn it_adds_two() {
    println!("It's adding two");
    let result = add(2, 2);
    assert_eq!(result, 4);
}