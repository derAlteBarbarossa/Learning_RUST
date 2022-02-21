#![allow(unused_variables)]
#![allow(dead_code)]

struct User<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> User<'a> {
    fn longest (&'a self, other: &'a User) -> &'a str {
        if self.name.len() > other.name.len() {
            self.name
        } else {
            other.name
        }
    }
}

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

    let name1 = "Hassan";
    let user1 = User {
        name: &name1,
        age: 14,
    };

    let name2 = "Foad";
    let user2 = User {
        name: &name2,
        age: 2,
    };

    println!("{} is longer", user1.longest(&user2));
}
