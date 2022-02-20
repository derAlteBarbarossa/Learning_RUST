use std::collections::HashMap;

enum ValTypes {
    Int(i32),
    String(String),
}

fn main() {
    let v = vec![
        ValTypes::Int(3),
        ValTypes::String(String::from("kir")),
    ];

    /*
    let hello = "Здравствуйте";
    let s = &hello[0..3];

    println!("{}", s);
    */
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Kir"), 10);
    map.insert(String::from("Kir1"), 15);

    for (key, value) in map {
        println!("{}: {}", key, value);
    }

    /*
    let name = String::from("Kir");
    let score = map.get(&name);

    println!("{} has {:?}", name, score);
    */
}
