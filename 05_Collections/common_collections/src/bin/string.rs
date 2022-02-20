#[allow(unused_variables)]
fn main() {
    // Initialise a String with a string slice
    let s: String = "string".to_string();

    // Initialise a String with `from` method
    let s: String = String::from("string");

    // Concatenate a string slice to a String
    let mut s: String = String::from("");
    s.push_str("string slice concatenated");

    // Concatenate with `+` operator
    // Ownership of s1 is moved to s3, as it concatenates
    // s2 to s1 and returns the reference of s1 to s3
    let s1: String = String::from("string1 "); 
    let s2: String = String::from("string1 "); 
    let s3: String = s1 + &s2;

    // Concatenation without moving the Ownership
    let s1: String = String::from("string1 "); 
    let s2: String = String::from("string1 "); 
    let s3: String = format!("{}{}", s1, s2);

    // Rust stores strings UTF-8 formatted. Therefore,
    // indexing in a String is impossible!
    // let c1: char = String::from("تحت قميصي قلب تحت قميصك حجر")[0];

    // To iterate over UTF-8 codes of a string
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{}", c);
    }
    
    // To iterate over raw bytes of a string
    let hello = "مرحبا";
    for c in hello.bytes() {
        println!("{}", c);
    }
}
