fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("ABCDEFG");
    let string2 = "abcd";

    let result = longest(string1.as_str(), string2);

    println!("result: {}", result);
}
