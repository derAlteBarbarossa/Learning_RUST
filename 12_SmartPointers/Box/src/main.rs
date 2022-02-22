// We use Box as indirection, when creating recursive types:
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {

    fn traverse(self) {

        let mut head: Box<List> = Box::new(self);
        println!("{}", head.get_value().unwrap());
        
        while let Some(next_node) = head.next() {
            match next_node.get_value() {
                Some(value) => {
                    println!("{}", value)
                },
                None => {
                    return
                }
            }

            head = next_node;
        }
    }

    fn next(self) -> Option<Box<List>> {
        match self {
            Cons(_, item) => {
                Some(item)
            },

            Nil => {
                None
            }
        }
    }

    fn get_value(&self) -> Option<i32> {
        match self {
            Cons(item, _) => {
                Some(*item)
            },
            Nil => {
                None
            }
        }
    }
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    // println!("the first element: {}", list.get_value().unwrap());
    list.traverse();
}