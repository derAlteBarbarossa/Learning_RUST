# Pattern Matching

Patterns are a special syntax in Rust, instructing the program how to decompose our complex variables and control the flow of the program based on the pattern of variable.

Patterns have different type, which we are going to have a short review of:

## Destruct Variables
We define how to interpret a variable, like:
 
```rust
	let PATTERN = VARIABLE;
```

Here, the elements of our tuple would be assigned to x, y, and z, respectively.

## Ranges
In conjunction with a `match` expression:
```rust
	match VARIABLE {
		RANGE1 => {/* ... */},
		RANGE2 => {/* ... */},
		_ => {/* ... */},
	}
```

`_` is a very specific pattern, representing a placeholder, which would match to all cases.

## Return Values of Functions
For example if a function. returns a `Result<OK, Err>` and we are sure that it won't fail and would like to extract the value wrapped in`Ok`: 
```rust
	let r: Result<value, _> = f();

```

## if let
```rust
	if let PATTERN1 = VALUE {
		// do stuff
	} else if let PATTERN2 = VALUE {
		// do stuff
	} else {
		// do stuff
	} 
```

## while let
We can use `let` keyword with `while` to indicate the ending condition:

```rust
	while let PATTERN = VALUE {
		// do stuff
	}

```

## for loop
Using `iter()`, we can convert a list to an enumerated list and iterate over it.

```rust
	for PATTERN in VALUE {
		// do stuff
	}
```

## Function Parameters
Just like decomposing:

```rust
	fn <function name> (VALUE: PATTERN) {
		// do stuff	
	}
```