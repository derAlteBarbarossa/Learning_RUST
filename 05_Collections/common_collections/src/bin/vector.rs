// We can define an Enum type to store
// different types.
enum ValTypes {
    Int(i32),
    String(String),
}

fn main() {
    // Defining and initialising a vector
    let v: Vec<i32> = vec![1,2,3];

    // Defing a vector
    // It need type annotation
    let v: Vec<String> = Vec::new();

    // All elements of a vector should be of the
    // same type. Following line causes a compile-error
    // let v = vec![1, 1.5, "Hello"];

    // As a walkaround, we can define an Enum type to represent
    // different types a vector can accomodate.
    let v = vec![
        ValTypes::Int(3),
        ValTypes::String(String::from("Enums are cool")),
    ];

    
    // The following code would not compile.
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // Due to ownership rules, we can't have both
    // mutable and immutable references to a value:
    // let mut second_mut = &mut vec[1];

    // This would work neither, as v.push() takes a mutable
    // reference to the last element.
    // v.push(6);
    // println!("The first element is: {}", first);
}
