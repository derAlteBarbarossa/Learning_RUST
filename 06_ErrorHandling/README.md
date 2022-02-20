# Error Handling
Another amazing commitment to reliability of Rust is forcing the programmer to acknowledge the posibility of errors and taking appropraite actions as a response. This mandate makes our programs more robust by ensuring all errors are going to be handled.

In Rust's terminology, we have two major categories of errors:
1. Unrecoverable errors: always symptoms of bugs. In this case, we should halt the normal execution of program and inform the user.
2. Recoverable errors: Sometimes, a failure can easily be interpreted and get a response.

## Unrecoverable Error
In this case, we have to stop the execution. It can happen in two different ways:
1. Panicing: When a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters

However, as this cleaning up is costly, we can immediately

2. Abort the program without cleaning up and leave the rest to the operating system.

If you'd rather have aborting than unwinding, add the following line to your `[profile]` section in `Cargo.toml`:

```
[profile.release]
panic = 'abort'
```

In either case, the API is `panic!`. For example:

```rust
fn main() {
    panic!("crash and burn");
}
```

When a Rust program panics, it prints out the **Backtrace** of the execution, as a hint to the programmer to find out the resource of error. **Backtrace** is a list of all the functions that have been called to get to this point.

## Recoverable Erros
Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to. (like opening a file that doesn't exist. In this case, we can create that file.)

When writing a function, which might fail to perform its duty in a recoverable manner, it's a very good practice to return a value of `Result` type, in order to signal the caller function:

```rust
enum Result<T, E> {
    Ok(T),
	Err(E),
}
```
T represents the type of the value that will be returned in a success case within the `Ok` variant, and E represents the type of the error that will be returned in a failure case within the `Err` variant.

With an appropriate `match` we can extract the wrapped value in `Ok`. This might be a bit overwhelming. Hopefully, `Result<T, E>` has a good number of helper functions to help us with this:
1. `unwrap`: If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us
2. `expect`: similar to unwrap, but with a customised panic! error message

```rust
use std::fs::File;

fn main() {
	let f = File::open("hello.txt").unwrap();
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

## Propagating Erros
When you’re writing a function whose implementation calls something
that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do.

```rust
use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error>
{
	let f = File::open("hello.txt");
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
    	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}
```

The `?` sortcut for error propagation works in almost the same way as the match expressions we defined to handle the Result values above: If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.


```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}
```
Just keep in mind that the `?` shortcut is usable only in functions with `Result` return value.

