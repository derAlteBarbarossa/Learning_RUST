# Ownership

As you will see later, **Ownership** is truely the most unique feature of Rust programming language, enabling Rust to make ememory safety guarantees even in case of concurrent execution.

## What's Ownership System in Rust?

Ownership system is a set of compile-time contraints, checked by compiler, which would gurantee memory safety, when satisfied. 

You probably have some painful experiences of debugging a code to find some subtle bugs, like dangling references, double-freeing, data races, etc. Don't worry anymore! Rust is here to help you. But, this advantage comes with a cost: getting used to ownership system. 

It would be really annoying at the very beginning, that violating these rules would cause **compile error**. However, after getting used to it, it would not only be simple to adhere to these rules, but also you would have a pleasant feeling of safety!

## What are these Constraints?

Rust ownership system enforces 3 rules to be satisfied:
1. Each **Value** should have on **Owner**
2. at any point of time, each value can have **only ONE** owner
3. When the owner goes out of scope, the corresponding value would be **Dropped**

## Move Ownership

Look at this example:

```
	let s1 = String::from("Hello, Ownership!");
	let s2 = s1;

```

What happens under the hood, is that first rust writes the string "Hello, Ownership!" somewhere in the heap memory, then assigns a reference(`s1`) to this literal on the stack memory. However, after `let s2 = s1`, Rust does a shallow copy and instead of actually copying the contents of `s1` into `s2`, makes `s2` point to the contents of `s1`. Finally, **it invalidates the reference from s1**. Which means if we later try to access `s1`, it would throw a compile error.


To prove it, you can simply print out `s1`'s content,

```
	println!("{}", s1);
```

and get a lovely error!


Technically speaking, for non-trivial values, like strings, structures, enumerations, etc. Rust does a shallow copy and **moves** **ownership** of the underlying **value** to another **variable** . In the example above, the ownership of string "Hello, Ownership!" has been moved to `s2` and `s1` can no longer claim it.


### When does the ownerhip move?

Before going on, please keep in mind, that we're talking about non-trivial values!

In general you would either move the ownership **explicitly** by calling `move` (we would see examples of this in multi-threading examples later) or **implicitly**. This implicit move happens in three cases:
1. Function calls: ownership of passed arguments would be moved from caller to the callee scope
2. Return values: in this case, ownership would be moved from callee to the caller scope
3. Assignments

## References
You find these rules so restrictive, that you might think they even limit the functionality of a program. As an example, how can we pass a string to a function to modify and return it? That's right. Hopefully, there's a very intuitive solution for that, which is called **Reference** in the Rust's terminology.

What's a reference? As the name suggests, Reference is a reference to a value without owning it. Let's see an example:

```
	let s1 = String::from("Hello, Ownership!");
	let s2 = &s1;
```
Now, `s2` refers to the same value as s1 and we can access both of the **immutably**!

## Mutable References
We can easily have a mutable reference to a value, if these three rules are satisfied:

1. We can have as many as immutable references,
2. We can have **only one mutable** reference to a value, and
3. We can't have references of both **mutable** an **immutable** types.

If you think about it carefully, you realise this would avoid any Data Race.