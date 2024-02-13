fn main() {
    about_scopes();
    println!();
    about_ownership()
    println!();
    about_references();
}


fn about_scopes() {
    let x: i32  = 1;
    println!("scope: 01 - x = {x}, X is available here because it's pertencent to this scope! ");
    {
        println!("scope: 02 - x = {x}, X is available here too because this scope that initiates with '{{' and ends with '}}' are a child scope and x was declared before the scope initalization");

        let y: i32 = 2;
        
        println!("scope: 02 - y = {y}, Y are the same as X, existing in this scope.")
    }

    println!("scope: 01 - Now X are still available here, but not Y, because Y do not exist in this scope! x = {x}");
    println!("scope: 01 - Since Y are not available in this scope, after the '}}' in line 15, the y variable will be droped by rust, no longer being saved in the memory")
}

fn about_ownership() {
    let _imut_string: &str = "This hardcoded string is immutable!";
    let mut mut_string: String = String::from("This string is a mutable string");

    {
        let takes_ownership: String = mut_string;

        println!("When you declare and assing a variable like the 'let takes_ownership = mut_string', the ownership goes from mut_string to takes_ownership.");
        println!("Now if you try to use the mut_string throws a compiler error saying isn't possible to use that var.");
        println!("You can use takes_ownership normally because now has the ownership. takes_ownership = {takes_ownership}");
        
        mut_string = takes_ownership // gives back the ownership to mut_string
        // if you don't gives back the ownership after the end of scope takes_ownership will be droped and mut_string will not be reusable.
    }

    
    println!("{mut_string}");

}

fn about_references() {
    println!("To avoid some problems with ownership we can use references...");
    let mut mut_string = String::from("Another imutable string");

    {
        let takes_ownership = &mut_string;
        println!("Use the symbol amperstand (&) to create a reference...");
        println!();

        println!("Now takes_ownership and mut_string are usable at same time. takes_ownership is a immutable reference!!");
        println!("takes_ownership = '{takes_ownership}' and mut_string = '{mut_string}'");

        println!();
        println!("With this reference, when rust drop takes_ownership, there is no need to pass ownership back to mut_string")
    }
    {
        println!();
        println!("You can't mutate a immutable reference.");
        println!("But there is a way to create mutable references.");

        let mutable_borrow_variable = &mut mut_string; // mut_string must be declared as let mut
                                                       // mutable_borrow_variable.push_str(" This can be mutable");
                                                       // this mutate the original variable mut_string!!
        println!("{0}", mutable_borrow_variable);
    }
    {
        println!();
        println!("You can have any amount of immutable references as you need, but only one mutable reference");

        let immutable_one = &mut_string;
        let immutable_two = &mut_string;
        let immutable_three = &mut_string;
        //let mutable_reference = &mut mut_string; <- This after three immutable references will throw a compiler error because the refs will be used

        println!(
            " immut one: {0}\n immut two: {1}\n immut three: {2}",
            immutable_one, immutable_two, immutable_three
        );

        let mutable_reference = &mut mut_string;
        mutable_reference.push_str("Pushed String");
        println!(" Now you can use a mutable reference if there's no immutable reference being used or will be used later, {mutable_reference}")
    }
}
