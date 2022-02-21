# Iterators

Iterators are a pattern allowing us to perform some tasks on a sequence of items. An iterator abstracts away the details of iteration and only exposes a logical interface to iterate over items.

## Creating an Iterator
It's important to mention that iterators are **lazy** in Rust, meaning that they have no effects until we call methods on them to consume the values in a sequence.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

Now, the iterator is stored at `v1`, however, no action has yet taken place. To access elements of this vector, we have to iterate over it:

```rust
for val in v1_iter {
    println!("Got: {}", val);
}
```

`for` loop takes ownership of the iterator.

## Iterator Trait
An object is an iterator, if it implements `Iterator` trait in the standard library of Rust.
```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```

We define `type` of the values this trait is going to iterate over them as `Item`.
The `Iterator` trait only requires implementation of one method, `next`, which returns the elem ents of the vector one-by-one until it reaches the end and, then returns `None`.

Iterators need some state to keep track of the current element in the vector they're processing. When calling `next` method, this internal state of the iterator would be changed. Hence, we need to define our iterator as `mut`.

We can categorise iterators, based on mutability and move of ownership, into 3 types:
1. immutable references to the values in vector => `iter()`
2. taking ownership of the vector and return owned values => `into_iter()`
3. mutable references to the values in vector => `iter_mut()`

Methods in the `Iterator` trait, are of two general types:
1. consuming adaptors: they call `next` and therefore consume the iterator.
```rust
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

```
`sum` has consumed our iterator `v1.iter()`. Consequently, accessing `v1.iter()` would cause us a compile error.

2. iterator adaptors: they allow us to change iterators to other types of iterators. One very practical use case is **chaining** iterators. However, as the iterators are lazy, we have to call a consuming adaptor to get result of this chain.

```rust
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
assert_eq!(v2, vec![2, 3, 4]);
```

In this case, `collect` consumes the iterator and *collect*s the results into a collection.