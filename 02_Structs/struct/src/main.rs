// derive keyword instructs Rust compiler to generate
// some predefined methods. For example here, we want
// a method to dump all fields of the struct. For this
// purpose we use Debug.
#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    user_name: String,
    password: String,
    email: String,
    age:    u8
}

impl User {
    // Methods
    // Methods all take `self` as the first parameter, as
    // they are accessing our struct
    fn check_password(&self, entered_pass: &str) -> bool {
        self.password.eq(entered_pass)
    }

    // Here, we are going to update a field of our struct,
    // which cmeans we are update our struct. Therefore,
    // we define `&mut self` as the first parameter
    fn update_password(&mut self, new_password: &str) {
        self.password = String::from(new_password);
    }

    // Associated Function
    // They are like helper functions.
    fn new(first_name: &str, last_name: &str, pass: &str, email: &str, age: u8) -> Self {

        User {
            first_name: first_name.to_string(),
            last_name:  last_name.to_string(),
            user_name:  User::generate_username(first_name, last_name),
            password:  pass.to_string(),
            email   :  email.to_string(),
            age
        }
    }

    fn generate_username(first_name: &str, last_name: &str) -> String{
        let mut user_name: String = String::from("");
        user_name.push_str(first_name);
        user_name.push_str(last_name);

        return user_name
    }
}

fn main() {
    // 1. As we are going to modify `new_user`, we have to
    //  instantiate it as mutable
    // 2. `new` is an associated function and should be called
    //  by ::
    let mut new_user = User::new("Hasan", "Pashmak", "12345", "hasanpashmaki@hajabdullah.com", 12);
    println!("{}", new_user.check_password("54321"));

    new_user.update_password("54321");
    println!("{}", new_user.check_password("54321"));

    //  When dereived a Debug trait for a struct, we can dump
    // its fields by :?. 
    println!("{:?}", new_user);
}
