use std::fs;
use std::io::ErrorKind;
use std::fs::File;

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

pub fn recoverable_with_results() {
    println!("Most errors aren't serious enough to require the program to stop entirely.");
    println!("Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.");

    /*
    enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    */

    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Error reading file, {:#?}", error),
    };

    // Matching on diferent errors

    let greeting_file_result: Result<File, std::io::Error> = File::open("./inexistent_file");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/lessons/class_06_error_handling/existent_file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    println!("{:#?}", greeting_file);

    fs::remove_file("./src/lessons/class_06_error_handling/existent_file.txt").expect("Ops");
}

