// This function takes ownership of its
// arguments
fn take_ownership(string: String) {
    println!("passed in {}", string);
}

// This is a trick to take the ownership of a value
// and turn it back!
fn take_and_give_ownership(string: String) -> String {
    // println!("passed in {}", string);
    string
}

// The most standard way is to se references
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let mut s = String::from("Hi!");
    //take_ownership(s);
    //s = take_and_give_ownership(s);
    let length = calculate_length(&s);

    println!("String: {} has length: {}", s, length);
}