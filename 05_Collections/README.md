# Common Collections

Collections are a way to pack multiple values together. The data stored in collections is stored in the heap and a reference to them would be stored on the stack.

Collections have different type, some of which we are going to have a short review of:

## Vector
`vec<T>` can only store values of the same type, `T`. We use them when conceptually we have a list of items.

When creating a new vector without initialisation, we need type annotation:
```rust
let v: Vec<usize> = Vec::new();
```
And if we want to initialise them while creating:
```rust
let v = vec![1, 2, 3];
```
To add to an element to a vector, first we have to define it as mutable and then use `push()` method:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
```

To index an element, we have to option:
1. indexing syntax:

```rust
let third: &i32 = &v[2];
```

2. get(), which returns Some(T)
```rust
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

Iterating over a vector can be done either immutably:
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
or mutably
```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
	*i += 50;
}
```
## String
The String type, which is provided by Rust’s standard library, is a **growable**, **mutable**, **owned**, **UTF-8** encoded string type.

Creating a new string:
```rust
let mut s = String::new();
let data = "initial contents";
let s = data.to_string();
```

Creaton with initialisation:
```rust
	let s = String::from("initial contents");
```

Pushing string slice/character to end of a string:
```rust
let mut s1 = String::from("foo");
let s2 = "bar";
// push a string
s1.push_str(s2);

//push a single character
s1.push('l');
```
Rust strings don’t support indexing, as strings are UTF-8 encoded.

## Hash Map
The type HashMap<K, V> stores a mapping of keys of type K to values of type V. It does this via a hashing function, which determines how it places these keys and values into memory.

to bring hash map library into scope, you need to include it:

```rust
use std::collections::HashMap;
```

Creating a new hash map:
```rust
let mut scores = HashMap::new();
```

Inserting to a hash map:
```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

To create and initialise a hash map, we have to store keys and values in two different vectors, first `zip` them together and `collect` them at the end. Keep in mind that type annotation is necessary!
```rust
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

For types that implement the Copy trait, such as i32, the values are copied into the hash map. For owned values such as String, the values will be moved and the hash map will be the owner of those values.

To access values in a hash map, we use `get` method. This method returns `Option<&V>`, as there's possibility that the corresponding value doesn't exist in the hash map.
```rust
let score = scores.get(String::from("Blue"));

```

If we want to insert a <key, value> only if the key doesn't exist, we have to do it like this:

```rust
scores.entry(String::from("Yellow")).or_insert(50);
```

`or_insert` method returns a **mutable** reference to the value if the key exists, otherwise inserts its argument value.
