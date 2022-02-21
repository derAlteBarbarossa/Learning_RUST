# Smart Pointers

References are pointers to an address in memory, indicated by the `&` symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data.

Smart pointers extend normal pointers, as they embed data structures that have additional metadata and capabilities.

## Box<T>
Box is like a normal reference. However, as they store they data on the heap and the only thing that remains on the stack is just a pointer (remember that pointers size are fixed and architecture dependent), we can use them to store values of many different types.

In general, we use `Box` in 3 different scenarios:
1. When we have a type whose size can’t be known at compile time
2. When we have a largely-sized data and don't want to copy it as a whole but just trnasfer the onwership to another process
3. When we want to bound our values to be of those types implementing a particulat trait

```rust
fn main() {
	let b = Box::new(5);
	println!("b = {}", b);
}
```

In the above example, we have stored a value of `5` on the heap, and put a reference to it on the stack.

## Rc<T>
In some cases, we need to share one value amongst many owners. To enable *multiple ownership*, Rust has introduced a type called `Rc<T>`, which is stands for **Reference Counting**, and keeps track of the number of references to a value. When this number reaches zero, runtime realises that this reference is no more in use and can safely drops it.

To get a new reference of `Rc<T>`, we call `Rc::clone`, which increases the counter of `Rc<T>` by one.

```rust
enum List {
	Cons(i32, Rc<List>),
	Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
	let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	println!("count after creating a = {}", Rc::strong_count(&a)); // 1

	let b = Cons(3, Rc::clone(&a));
	println!("count after creating b = {}", Rc::strong_count(&a)); // 2

	{
		let c = Cons(4, Rc::clone(&a));
		println!("count after creating c = {}", Rc::strong_count(&a)); // 3
	}

	println!("count after dropping c = {}", Rc::strong_count(&a)); // 2
}
```

## RefCell<T>
Technically, simultaneously having both mutable and immutable valid references to a value is not allowed. However, as Rust compiler is very conservative with borrowing rules, in many cases it complains about violation. But there are some cases, that a programmer knows that reading and writing to a variable would not happen at the same time, or in other words data race is impossible. With `RefCell<T>` we can postpone this borrow checking to runtime. If during the execution flow, any of these rules break, the program would panic and exit.

`RefCell<T>` provides us with **interior mutability**, even if the `RefCell<T>` variable is defoned as immutable, its interior value can is mutable. Given a `RefCell<T>` variable, we can cast it to `Ref<T>` by calling `borrow` and to `RefMut<T>` by calling `borrow_mut`.

```rust
fn main() {
	let b = RefCell::new(vec![1,2,3]);
	
	let r1 = b.borrow();
	// The following line would panic at runetime
	// as b is already borrowed as immutable 
	let mut r2 = b.borrow_mut();

	for value in r1.iter() {
	    println!("{}", value);
	}
	

	for value in r2.iter_mut() {
	    *value += 2;
	    println!("{}", value);
	}

}
```

We can fix it, if we seperate the scopes:

```rust
fn main() {
	let b = RefCell::new(vec![1,2,3]);
	
	{
		let r1 = b.borrow();

		for value in r1.iter() {
			println!("{}", value);
		}
	}
    
	{
		let mut r2 = b.borrow_mut();

		for value in r2.iter_mut() {
			*value += 2;
			println!("{}", value);
		}
	}
}
```
## Deref Trait

By implementing the `Deref` trait, we can have a customised behaviour of the dereference operator, `*`.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;
	
	fn deref(&self) -> &T { 
		&self.0
	}
}
```

Without the `Deref` trait, the compiler can dereference only & references. The `Deref` method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.

## Drop Trait

`Drop` lets us customise what happens when a value gets out of scope. The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to self.

```rust
struct CustomSmartPointer {
       data: String,
}

impl Drop for CustomSmartPointer {
	fn drop(&mut self) {
		println!("Dropping CustomSmartPointer with data `{}`!", self.data);
	}
}

fn main() {
	let c = CustomSmartPointer { data: String::from("my stuff") };
	let d = CustomSmartPointer { data: String::from("other stuff") };
	println!("CustomSmartPointers created.");
}
```
Variables are dropped in the reverse order of their creation.


















