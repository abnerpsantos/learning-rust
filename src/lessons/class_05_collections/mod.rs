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
