pub fn about_tests() {
    println!("At its simplest, a test in Rust is a function that's annotated with the test attribute.");
    println!("Attributes are metadata about pieces of Rust code;");
    println!("To change a function into a test function, add #[test] on the line before fn");
    println!("Run 'cargo test' to test all functions that have the test attribute. ");
}


#[test]
fn it_works() {
    let result: i32 = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn it_doesnt_works() {
    let result: i32 = 2 + 3;
    assert_eq!(result, 4);
}
