fn main() {
    /*
    * It works, as we are using String type
    * Memory requested from OS at runtime
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
    
    * Doesn't work, as we are using string literals and they don't support push_str
    * We have hardcoded the string at the compile-time
    let mut s = "Hello";
    s.push_str(", world!");
    println!("{}", s);
    
    * Ownership moved to s_2
    let s_1 = String::from("Hello");
    let mut s_2 = s_1;
    println!("{}", s_1);
    
    * Ownership remains in s_1
    let s_1 = String::from("Hello");
    let mut s_2 = s_1.clone();
    s_2.push_str("kir");
    println!("{}", s_1);
    */
}
