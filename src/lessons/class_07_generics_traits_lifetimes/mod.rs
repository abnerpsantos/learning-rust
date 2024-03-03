use std::fmt::Debug;

pub fn about_generics() {
    println!("Every programming language has tools for effectively handling the duplication of concepts.");
    println!("In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.");
    println!("We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.");

    {
        // Instead of doing this: :
        fn _return_string(string: String) -> String {
            string
        }
        fn _return_i32(value: i32) -> i32 {
            value
        }

        // we can use generics:

        fn return_value<T>(value: T) -> T {
            value
        }

        let val_i32: i32 = return_value::<i32>(43);
        let val_string: String = return_value::<String>(String::from("Hello World!"));

        println!("val_i32: {}, val_string: {}", val_i32, val_string);
    }
    {
        println!("\nStructs with generics");

        #[allow(dead_code)]
        #[derive(Debug)]
        struct Values<T, K, V> {
            t: T,
            k: K,
            v: V,
        }

        let values: Values<i32, f64, String> = Values::<i32, f64, String> {
            t: 32,
            k: 3.45,
            v: String::from("Hello World!"),
        };
        println!("{:#?}", values);
    }
    {
        println!("\nWith Enums:");

        #[derive(Debug)]
        enum Values<T, K> {
            T(T),
            K(K),
            TK(T, K),
        }

        let values_t: Values<f32, ()> = Values::T(15.54);
        let values_k: Values<(), u32> = Values::K(64);
        let values_tk: Values<i32, &str> = Values::TK(31, "Hello");

        println!("{:#?},{:#?},{:#?}", values_t, values_k, values_tk);
    }

    println!("By using generics we can do more with less code!!");
}

pub fn about_traits() {
    println!("A trait defines functionality a particular type has and can share with other types.");
    println!("We can use traits to define shared behavior in an abstract way.");
    println!("A type's behavior consists of the methods we can call on that type.");
    println!("Different types share the same behavior if we can call the same methods on all of those types.");
    println!("Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.");

    trait AnimalTraits {
        fn eat(&self) -> String;
        fn make_sound(&self) -> String;
    }

    struct Animal {
        pet: String,
        sound: String,
        food: String,
    }

    impl AnimalTraits for Animal {
        fn eat(&self) -> String {
            format!("{} is eating {}", &self.pet, &self.food)
        }
        fn make_sound(&self) -> String {
            format!("{} does: {}", &self.pet, &self.sound)
        }
    }

    let dog: Animal = Animal {
        pet: String::from("Dog"),
        food: String::from("Dog food"),
        sound: String::from("Bark"),
    };
    let cat: Animal = Animal {
        pet: String::from("Cat"),
        food: String::from("Cat food"),
        sound: String::from("Meow"),
    };

    fn print_animal_traits(pet: &impl AnimalTraits) {
        println!("\n{}", pet.make_sound());
        println!("{}", pet.eat())
    }

    print_animal_traits(&cat);
    print_animal_traits(&dog);

    // Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
}
