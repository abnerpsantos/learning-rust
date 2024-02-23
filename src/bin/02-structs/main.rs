
fn main(){
    about_structs();
    tuple_structs();
    method_syntax();
    method_syntax_exercise();
    about_constructor_functions();
}

#[allow(dead_code)]
fn about_structs() {
    println!("Structs are similar to tuples, both can hold multiple related values, but for structs you need to name the fields");
    struct User {
        name: String,
        age: i32,
        logged_in: bool,
    }

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

    let _black: Color = Color(0, 0, 0);
    let _origin: Point = Point(0, 0, 0);

    /*
    Note that the black and origin values are different types because they’re instances of different tuple structs. 
    Each struct you define is its own type, even though the fields within the struct might have the same types. 
    For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. 
    Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . (dot) followed by the index to access an individual value.
     */

}

#[allow(dead_code)]
fn method_syntax() {
 /*
 Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
 Unlike functions, methods are defined within the context of a struct (or an enum or a trait object
  */

  struct Rectangle {
    width: u32,
    height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
  }

  let rect_one: Rectangle = Rectangle {
    width: 5,
    height: 5
  };

  println!("the area of a rectangle with width: {0} and height: {1} is {2}", rect_one.width, rect_one.height, rect_one.area())
}

#[allow(dead_code)]
fn method_syntax_exercise() {
    struct Rectangle {
        width: u32,
        height: u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, rectangle: &Rectangle) -> bool {
            self.area() > rectangle.area()
        }
    }

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[allow(dead_code)]
fn about_constructor_functions () {
    println!("All functions defined within a implementation block (impl) are called associated funcions.");
    println!("Functions that do not need the self parameter, and thus are not methods, often are used as constructor functions");
    println!("Constructor function returns a new instance of the struct.");

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height
            }
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn can_hold(&self, rectangle: &Rectangle) -> bool {
            self.area() > rectangle.area()
        }
    }

    /*
    creating a rectangle instance was:
        let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    instead you can use the constructor function:

     */
    let rect_one: Rectangle = Rectangle::new(30,50);
    let rect_two: Rectangle = Rectangle::new(10, 40);

    println!("Can rect1 hold rect2? {}", rect_one.can_hold(&rect_two));
}
