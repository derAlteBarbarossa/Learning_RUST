# Structs
Structs are a way to pack related data together. Rust provides use with two different types of structs:
1. (Normal) Struct: To access a subfield, we need to proved subfield's name:
```
struct User {
    first_name: String,
    last_name: String,
    user_name: String,
    password: String,
    email: String,
    age:    u8
}

let user: User = User::new(...);
user.age;
```
2. Tuple Strcut: To access a subfield, we have to index it:
```
struct Colour (uint8, uint8, uint8);
let black = Colour(0,0,0);
```

## Struct Initialisation

To create a new struct instance, we can either
1. Define an instance with subfields provided:

```
struct Circle {
    centre_x: u32,
	centre_y: u32,
	radius:   u32
}

let c: Cricle = Circle {
	centre_x: 5,
	centre_y: 20,
	radius:	  7
};

```

2. Use a build associated function

```
let c: Circle = Circle::new(5, 20, 7);

```

if you want to use the second method, you have to implement the associated function under `impl` block, which we are going to discuss next.

## impl Block
`impl` block describes the API presented to the programmer to manipulate struct's data or do some required functionality. It's syntax is:

```
struct Circle {
    centre_x: f32,
	centre_y: f32,
	radius:   f32
}

impl Circle {
	// Associated function		
	fn new(centre_x: f32, centre_y: f32, radius: f32) -> Self {
		Circle {
			centre_x,
			centre_y,
			radius
		}
	}	

	// Method
	fn area(&self) -> f64 {
		PI*radius*radius
	}
}
	
```

### Associated Function & Methods
Methods are coupled with our struct, while associated functions are more like helper function to facilitate computation. When calling a method, we use **dot** notation, for example

`<struct name>.<method name>(parameters)`

In contrast, when calling an associated function, we use **double colon**:

`<struct name>::<function name>(parameters)`