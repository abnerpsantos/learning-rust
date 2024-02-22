
struct User {
    name: String,
    age: i32,
    logged_in: bool,
}

fn main(){
    // about_structs();
    tuple_structs();
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

#[allow(dead_code)]
fn tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    /*
    Note that the black and origin values are different types because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
     */

}
