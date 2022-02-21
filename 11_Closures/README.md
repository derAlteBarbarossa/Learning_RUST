# Closures

Closures are one of the Rust's facilities to write functional programs. They're anonymous functions, where we can save in a variable or pass the closure itself to another function.

One of very interesting, and performance boosting, techniques we can implement with closures is **memoization**. Memoisation is an optimization technique to speed up computer programs by
1. postponing the actual calculation until really needed
2. storing the results of expensive function calls and returning the cached result when the same inputs occur again, thus avoiding redundant calculation.

## Definition of a Closure

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

The closure definition comes after the `=` to assign it to the variable expensive_closure. To define a closure, we start with a pair of vertical pipes (`|`), inside which we specify the parameters to the closure. After the parameters, we place curly brackets that hold the body of the closure. It's worth of mentioning that this `let` assignment is just a definition and not the resulting value of the calculation.

## Type annotation on Closures
As closures are not exposed to the outer world, in the small context of a local file, compiler would be able to infer the types. Therefore, no types annotation is required. However, we can add annotation to make it more readable:

```rust
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

When we instantiate a closure, compiler automatically infers the type and locks the closure! For example, the following code would not compile:

```rust
let example_closure = |x| x;
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

as the fist call to the closure enforced the type of `x` to e `String`. We can't use the same instance for `integer`.

## Closure Types
Closure have access to the environmental variables. For example:

```rust
fn main() {
	let x = 4;
	let twice_x = |z| z == 2*x;
	let y = 8;
	assert!(twice_x(y));
}
```

When accessing environment, we should be careful about the ownership rules. Depending on our use case, we might need to define the closure as one of these types:

1. `Fn` borrows values from the environment immutably.
2. `FnMut` borrows values from the environment mutably.
3. `FnOnce` takes ownership of the variable, which it captures from the environment.

When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.

However, in some cases (like in multithreading), we might explicitly enforce the compiler to move the ownership, in these cases we would re-write the above code as:
```rust
fn main() {
	let x = 4;
	let twice_x = move |z| z == 2*x;
	// x is not accessible anymore!
	let y = 8;
	assert!(twice_x(y));
}
```
## Storing Closures
When storing a closure, like in a struct, we need to define its type. We do it by using trait bounds:

```rust
struct Cacher<T>
	where T: Fn(u32) -> u32
{
	calculation: T,
	value: Option<u32>,
}
```
Any closure we want to store in the calculation field must have one `u32` parameter and must return a `u32`.


