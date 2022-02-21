# Generics

One way to avoid duplication is functions. Depsite their major contribution to avoiding duplications, they have a very limiting problem; they're bounded to one concrete type.

Generics are a way to handle duplication of concepts in an abstract. We can express the behaviours withour knowing what concrete type will be in their place when running the code. Thus, allowing the code to operate on abstract type.

## Generics in Function Definition

We place them in the signature of the function:
```rust
fn largest<T>(list: &[T]) -> T {
	// do stuff
}
```

The `<T>` defines a generic type, which the parameter `list` is going to consume it and the function returns a value of type `T`.

## Generics in Struct Definition
First in the angle brackets, we define our generic type, which is going to be used for the subfields.
```rust
struct Point<T> {
	x: T,
	y: T,
}
```

We can alsi have different generic types:

```rust
struct Point<T, U> {
	x: T,
	y: U,
}
```
## Generics in Enum Definition
We can define enums to hold generic data types in their variants. Two very famous built-in Enums of this form are:

```rust
enum Option<T> {
	Some(T),
	None,
}
```

Option<T> is an enum that is generic over type T and has two variants: Some, which holds one value of type T, and a None variant that doesn’t hold any value. By using the Option<T> enum, we can express the abstract concept of having an optional value, and because Option<T> is generic, we can use this abstraction no matter what the type of the optional value is.


```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```
We use the Result enum anywhere we have an operation that might succeed (return a value of some type T) or fail (return an error of some type E).

## Generics in Method Definition

```rust
struct Point<T, U> {
	x: T,
	y: U,
}

impl<T, U> Point<T, U> {
	fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
		Point {
			x: self.x,
			y: other.y,
		}
	}
}
```

## Monomorphization

Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when **compiled**.

When Rust compiles this code, it performs monomorphization. During that process, the compiler reads the values that have been used in generic instances and those instances are replaced with specific definitions. As a result, we would have to performance penalty, when using generics.