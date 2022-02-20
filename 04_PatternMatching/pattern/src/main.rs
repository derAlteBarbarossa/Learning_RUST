fn main() {
    let age: Result<u8, _> = "34".parse();
    
    println!("Hello, world! {:?}", age);
}
