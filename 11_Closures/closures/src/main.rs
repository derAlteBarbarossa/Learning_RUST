use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// U is a generic type for keys
// V is a generic type for HashValue

struct Cacher<T, U, V> {
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V> 
    where T: Fn(U) -> V,
          U: Eq + Hash + Clone,
          V: Copy
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let key = arg.clone();
        match self.values.get(&key) {
            Some(value) => *value,
            None => {
                let value = (self.calculation)(arg);
                self.values.insert(key, value);
                value
            },
        }
    }
}

fn main() {
    let mut expensive_closure = Cacher::new(|input| {
        println!("Calculating Slowly");
        thread::sleep(Duration::from_secs(5));
        input
    });

    //expensive_closure.value(2);
    //expensive_closure.value(&"Helloo");
    let s1 = String::from("Helloo");
    let s2 = String::from("Hello");
    expensive_closure.value(&s1);
    expensive_closure.value(&s2);
    //expensive_closure.value(&"Hello");
    //expensive_closure.value(2);
}
