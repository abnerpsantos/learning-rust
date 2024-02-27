use std::collections::HashMap;

#[allow(dead_code)]
pub fn about_vectors() {
    println!("Rust's standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values.");
    println!("Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.");
    println!("The first collection types are: Vectors");

    let mut v1: Vec<i32> = Vec::new();
    let _v2: Vec<i32> = vec![1,2,3];

    v1.push(5);
    v1.push(4);

    println!("Reading elements of vectors, with get or indexes");
    let v1_i0: i32 = v1[0];
    let v1_i1: Option<&i32> = v1.get(5);

    println!("{:#?},{:#?},{:#?}",v1_i0, v1_i1, v1);
    // With Strings...

    let mut v3: Vec<String> = Vec::new();
    v3.push(String::from("1"));
    v3.push(String::from("2"));
    v3.push(String::from("3"));

    let v3_01: &String = &v3[0];

    println!("{:#?},{:#?}", v3, v3_01)

}

#[allow(dead_code)]
pub fn iterating_over_vectors() {
    println!("We can iterate over a vector with for loops...");
    let v: Vec<i32> = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!("To iterate over mutable vector a make changes of values we need to use a mut vec and derreference to get the value before change");
    let mut v: Vec<i32> = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:#?}", v)

}

#[allow(dead_code)]
pub fn storing_multiple_datatypes() {
    println!("When we create a vector, it's data type can only be of one type, i32, String, etc. ...");
    println!("But we can store enums with multiple types in vectors to create a multi data type vector...");

    #[derive(Debug)]
    struct StructType;

    #[derive(Debug)]
    enum Types {
        TString(String),
        Ti32(i32),
        TStruct(StructType)
    }

    let mut vec_of_types: Vec<Types> = Vec::new();
    vec_of_types.push(Types::TString(String::from("Hello")));
    vec_of_types.push(Types::Ti32(45));
    vec_of_types.push(Types::TStruct(StructType));

    println!("{:#?}", vec_of_types)
}

pub fn about_hash_maps() {
    println!("With hash maps we can store values with keys associated to them...");
    println!("The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);
}
