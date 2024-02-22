
struct User {
    name: String,
    age: i32,
    logged_in: bool,
}

fn main(){
    about_structs();
}

#[allow(dead_code)]
fn about_structs() {
    println!("Structs are similar to tuples, both can hold multiple related values, but for structs you need to name the fields");

    let abner: User = User {
        age: 23,
        name: String::from("Abner"),
        logged_in: true,
    };

    println!("name: {0}, age: {1}, is logged? {2}", abner.name, abner.age, abner.logged_in );
    println!("A field of an instance can only be mutable if the entire instance are mutable");
    /*
    struct User {
        name: mut String // not allowed
        age: i32
    }

    **Allowed**
    let mut user: User = User {
        name: String,
        age: i32
    }

    user.name = "Jon Doe"
     */
    let mut abner: User = User {
        name: String::from("Abner"),
        age: 23,
        logged_in: false
    };

    println!("name: {0}, age: {1}, is logged? {2}", abner.name, abner.age, abner.logged_in );
    abner.logged_in = true;
    println!("name: {0}, age: {1}, is logged? {2}", abner.name, abner.age, abner.logged_in );
}
