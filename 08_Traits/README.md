# Traits

A trait defines the behaviour and functionality a particular type implements and exposes a corresponding API to the other types and our program.

We can use **trait bounds** to specify that a generic can be any type with some certain and defined behaviour.

## Define a Trait
A type's behaviour composes all the methods we can call on that type. To define a trait, we have to define these behaviours:

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}
```
## Implementing a Trait
To implement a trait on a concrete type, we can use `impl` block:

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

**Important:** We can't implement external traits on external types. This restriction originates from **orphan rule**, because if we implement a trait on an external type, if this trait is already implemented in the external crate, rust would be confused and could not resolve which implementation to use.

## Default Implementation
Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
```rust
pub trait Summary {
	fn summarize(&self) -> String {
		String::from("Read more...");
	}
}
```
Syntax for overriding a default implementation is
the same as the syntax for implementing a trait method that doesn’t have a default implementation.

## Traits as Function Parameters
We can use traits to confine parameter types a function can accept. In this sense, only those types implementing this trait's functionality are accepted as input arguments.

```rust
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
Or in more verbose way:

```rust
pub fn notify<T: Summary> (item: T) {
    println!("Breaking news! {}", item.summarize());
}
```
Which means the function `notify` is defined over generic type `T` with a constraint that `T` should implement `Summary` trait.

We can also afford more complexity in implementing a trait:

```rust
pub fn notify<T: Summary>(item1: T, item2: T) {
	// do stuff
}
```

We can define multiple trait bounds, which means confining our types to implement more than one trait:

```rust
pub fn notify(item: impl Summary + Display) {
	// do stuff
}
```

Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. To solve this, Rust has introduced `Clauses`:

```rust
fn some_function<T, U>(t: T, u: U) -> i32
	where T: Display + Clone,
	U: Clone + Debug
{
	// do stuff
}
```

Using trait bounds, we can also put a constraint on the return type of a function:

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
	// do stuff
}
```

And the last use case, conditionally implementing a method:
```rust
use std::fmt::Display;
struct Pair<T> {
	x: T,
	y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
	fn cmp_display(&self) {
		if self.x >= self.y {
		println!("The largest member is x = {}", self.x);
		} else {
			println!("The largest member is y = {}", self.y);
		}
	}
}
```