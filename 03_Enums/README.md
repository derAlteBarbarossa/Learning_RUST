# Enums

Just like any other programming language, Rust has enum type to enumerate all possible values a variable can hold. To define an enum, we have to list all values it could take:

```rust
enum Week {
	Sunday,
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday
}
```

We can also define behaviour of this type by defining an `impl` block:

```
impl Week {
	fn is_weekend(&self) -> bool {
		return matches!(self, Week::Saturday) ||
			matches!(self, Week::Sunday)
	}
}
```

Pay attention that to access one enum value, we need to use **double colon** notation, like `Week::Friday`.

## Option<T>

This is a very special enum having one of two values:
1. Some(T): This value means that Option has some value
2. None: In this case, we encode non-existance of any values

Introducing this enum was a very samrt move by Rust development team. Instead of being worried about null-reference scenario (and probably leading to segmentation faults), we can check if a value has been assigned to this variable.