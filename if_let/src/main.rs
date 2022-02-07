//use std::collections::HashMap;

fn main() {
    /*
    let mut teams = vec![String::from("Blue"), String::from("Yellow")]; 
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("teams: {:?}", teams);

    teams.push(String::from("kir"));
    */
    let x = Some(5);

    if let Some(1) = x {
        println!("x is Some(1)");
    } else if let Some(5) = x {
        println!("x is Some(5)");
    } else {
        println!("x is Some(other)");
    }
}
