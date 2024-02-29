pub fn unrecoverable_with_panics() {
    println!("Rust has two ways of handle errors, recoverable errors and unrecoverable errors.");
    println!("Rust handle unrecoverable errors with the panics");
    println!("There are two ways to cause panic in practice, with actions that cause our code to panic, or the panic macro");

    let mut vec: Vec<i32> = Vec::new();
    vec.push(15);
    let existent_val = vec[0];
    // let _unexistent_vec_val = vec[1];
    // If we try to use _unexistent_vec_val, our code panics.
    // println!("{_unexistent_vec_val}"); -> Panics

    // Using the macro paninc!()
    match existent_val {
        15 => panic!("The value matched, so panic anyway..."),
        _ => panic!("Panic too...")
    }
}
