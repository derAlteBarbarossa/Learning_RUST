fn main() {
    /*
    let mut s = String::from("Hello!");

    change(&mut s);

    let r1 = &s;
    let r2 = &mut s;

    println!("{}", r1);
    */
    let s = "Hi!";
    s.chars().nth(0) = 'g';
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
