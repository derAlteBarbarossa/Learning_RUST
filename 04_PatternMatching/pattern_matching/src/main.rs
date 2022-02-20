#![allow(dead_code)]
#![allow(unused_variables)]

// Case 7:
// Use pattern matching to take appropriate actions
// for different cases of Option<T>
fn square(value: Option<f64>) -> Option<f64> {
    match value {
        Some(value) => Some(value*value),
        None => None,
    }
}

// Case 8:
// When matching with `match`, our code should be
// Exhaustive. 
fn is_weekend (day: &str) -> bool {
    match day {
        "Sunday" => return true,
        "Saturday" => return true,
        _ => return false
    }
}

// Case 9:
// Use pattern matching for function parameters
fn distance (&(x1, y1): &(f32, f32), &(x2, y2): &(f32, f32)) -> f32 {
    return f32::sqrt((x2-x1)*(x2-x1) + (y2-y1)*(y2-y1))
}

fn main() {
    // Case 1:
    // Use pattern matching to destruct
    let (x,y,z) = (1.0, 1.1, 3.2);

    // Case 2:
    // Use patterns for ranges
    let x = 12;
    match x {
        0..=12 => println!("Just a little kid"),
        13..=19 => println!("A teenager who thinks he is cool"),
        _ => println!("An old man"),
    }

    // Case 3:
    // parse() returns Result<F, F::Err>
    // we match the pattern to extract F
    // ::<> is called turbofish syntax to instruct the
    // compiler about the result type.
    let age: Result<u8, _> = "34".parse::<u8>();

    // Case 4:
    let x = Some(5);
    if let Some(1) = x {
        println!("x is Some(1)");
    } else if let Some(5) = x {
        println!("x is Some(5)");
    } else {
        println!("x is Some(other)");
    }

    // Case 5:
    let mut stack: Vec<u32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    while let Some(i) = stack.pop() {
        println!("stack top: {}", i);
    }

    // Case 6:
    let queue = vec![1,2,3,4,5,6];
    for (index, element) in queue.iter().enumerate() {
        println!("{}: {}", index, element)
    }

}
