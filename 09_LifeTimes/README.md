# Lifetimes
Lifetime is simply the scope, in which a reference is valid.
Most of the times, the Rust compiler can infer lifetime of a reference. However, there are very special, but fundamental scenarios, where inferring lifetime is hard or even impossible for compiler. In these cases, it's programmer's duty to signal the comiler about lifetimes of references.

## Why do we need Lifetimes?
The main purpose of lifetimes is to prevent **dangling references**. Dangling reference describes a situation, where program references an address in memory other than the intended address. For example:

```rust
{
	let r;
	{
		let x = 5;
		r = &x;
	}
	println!("r: {}", r);
}
```

This snippet would not be compiled, with the error `x does not live long enough`. as `x` is valid only in the inner block. When we take a reference from it and save in the r, as we go out of this block, `x` would be dropped automatically and `r` would no more be a valid reference to the value reperesnted by `x`.

The checking described above is accomplished by `borrow checker`, which compares the scopes to determine whether all references are valid or not.

## Generic Lifetimes
As a perfect example, consider the following function, which finds the longer string between two string slices and returns the longest one:


```rust
fn longest(x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}
```

When compiling we encounter `error[E0106]: missing lifetime specifier`. What does it mean?

When we’re defining this function, we don’t know whether the `if` or `else` will execute,as we don't know the concrete values of the input arguments. As these inputs might have different lifetimes, we can't infer whether the return reference would be valid.

## Lifetime Annotation in Function Signature
Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	// do stuff
}
```

The above syntax puts a lower bound on for how long references to `x` and `y` are valid. In the angle brackets, we define the life time `'a`. The parameter list `x: &'a str, y: &'a str`, we explicitly say that both `x` and `y` references are valid for at least `'a`. This signature, at the end, tell the Rust compiler the returned value would live at least for `'a`. As a result, compiler would understand the return value is valid at least as long as both `x` and `y` are valid and be ensured no dangling reference is going to threat our program.

When a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own.

## Lifetime Annotation in Struct Definition
When we wany to hold referenceswithin a struct, we would need to add a lifetime annotation on every reference in the struct’s definition.

```rust
struct ImportantExcerpt<'a> {
	part: &'a str,
}
```
This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its part field.

## Lifetime Elision Rules
They’re a set of rules, with help of which the compiler can infer lifetime, and if your code fits these cases, you don’t need to write the lifetimes explicitly:

1. Each parameter that is a reference gets its own life- time parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. Of there are multiple input lifetime parameters, but one of them is `&self` or `&mut self `because this is a method, the lifetime of self is assigned to all output lifetime parameters.

As you might suggest, these rules are a bit conservative. You can think of many scenarios, when these rules applied, they give a very low lower bound on lifetimes. Later, we would learn that programmers knowledge can further be leveraged in `unsafe` blocks to bring more flexibility. 

## Lifetime Annotations in Method Definitions
Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

```rust
impl<'a> ImportantExcerpt<'a> {
	/* ... */
}
```
Usually, lifetime annotation isn't necessary, as elision rules come to rescue: 3rd elision rule dictates that methods, which all take `&self` or `&mut self` as their first parameter, have lifetime of `&self`/`&mut self` as their return type lifetime. 

## Static Lifetime
One special lifetime we need to discuss is `'static`, which denotes the entire duration of the program. All string slices have a static lifetime, as their values would be stored directly in the binary of our program, therefore, always accessible.