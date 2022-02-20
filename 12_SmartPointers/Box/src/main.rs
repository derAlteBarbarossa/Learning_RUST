use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Weak<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Weak<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main()
{
    let a = Rc::new(List::Cons(5, RefCell::new(Weak::new())));
    let b = Rc::new(List::Cons(10, RefCell::new(Weak::new())));
  
    /*
    let c = Rc::new(List::Cons(15, RefCell::new(Rc::clone(&b))));

    println!("c next item: {:?}", c.tail());
    */

    //if let Some(link) = a.tail() { 
    //    *link.borrow_mut() = Rc::clone(&b);
    //}
    // Uncomment the next line to see that we have a cycle; // it will overflow the stack.
    //println!("a next item = {:?}", b.tail());
}