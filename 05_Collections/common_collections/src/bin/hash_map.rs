// To use HashMaps we need to include their library
use std::collections::HashMap;

fn main() {
    // Initialising a hash map is usually mutable
    // as we are going to insert values to it
    // HashMap<K, V>, where K is type of the key
    // and `V` is type of the value
    let mut grades: HashMap<String, i32> = HashMap::new();

    // Inserting a Key-Value pair
    let value: String = String::from("Hans");
    grades.insert(value, 5);
    // When `insert`ing, the ownership would be moved
    // to the hash map
    // This line causes a compile-error
    // println!("{}", value);

    // zip two arrays together to create a hash map
    // the first array should be keys and the second one
    // should be values
    let names = vec!["Ferdinand", "Leopold", "Wilhelm", "Friedhelm"];
    let scores = vec![5,6,6,4];

    // Type annotation needed here
    let scores: HashMap<_, _> = names.iter().zip(scores.iter()).collect();

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // When creating a hashmap with zipping, ownership
    // is not moved to the hash map.
    println!("{}", names[0]);

}
