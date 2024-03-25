use std::thread;

pub fn about_closures() {
    {
        println!("Rust's closures are anonymous functions you can save in a variable or pass as arguments to other functions.");
        println!("Unlike functions, closures can capture the enviroment");

        let random_var = 5;
        let closure_exemple = || {
            println!(
                "This is the random variable declared before the closure, {}",
                random_var
            )
        };
        closure_exemple();
    }
    {
        println!("\nClosures can capture the enviroment in three ways: borrowing mutably, borrowing immutably and moving ownership");
        {
            println!("\nThe immutable way");
            let list = vec![1, 2, 3];
            println!("Before defining closure: {:?}", list);

            let only_borrows = || println!("From closure: {:?}", list);

            println!("Before calling closure: {:?}", list);
            only_borrows();
            println!("After calling closure: {:?}", list);
        }
        {
            println!("\nThe mutable way");
            let mut list = vec![1, 2, 3];
            println!("Before defining closure: {:?}", list);
        
            let mut borrows_mutably = |v| list.push(v);

            borrows_mutably(52);
            borrows_mutably(32);
            println!("After calling closure: {:?}", list);
        }
        {
            println!("\nMoving ownership");
            let list = vec![1, 2, 3];
            println!("Before defining closure: {:?}", list);
        
            thread::spawn(move || println!("From thread: {:?}", list))
                .join()
                .unwrap();

            // list is no more available, captured by the closure
        }
    }
}
