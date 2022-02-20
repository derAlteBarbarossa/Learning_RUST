fn main() {
    /*

    Case 1:
    * Ownership moved to s_2
    * Would not be compiled!
    let s_1 = String::from("Hello, Ownership");
    let mut s_2 = s_1;
    println!("{}", s_1);

    Case 2:
    * Deep Copy
    let s_1 = String::from("Hello");
    let mut s_2 = s_1.clone();
    s_2.push_str(", Ownership");
    println!("{}", s_1);

    Case 3:
    * Passed reference to s_2
    * Ownership remains in s_1
    * Both references are immutable
    let s_1 = String::from("Hello, Ownership");
    let s_2 = &s_1;

    Case 4:
    * More than one mutable references
    * Would not compile
    let mut s_1 = String::from("Hello, Ownership");
    let s_2 = &mut s_1;
    let s_3 = &mut s_1;

    Case 5:
    * References of both immutable and mutable
    * Would not compile
    let mut s_1 = String::from("Hello, Ownership");
    let s_2 = &s_1;
    let s_3 = &mut s_1;
    println!("{}, {}", s_2, s_3);

    Case 6:
    * To borrow a value as mutable, it should be declared as mutable!
    * Would not compile
    let s_1 = String::from("Hello, Ownership");
    let s_2 = &mut s_1;
    println!("{}", s_2);
    */
}
