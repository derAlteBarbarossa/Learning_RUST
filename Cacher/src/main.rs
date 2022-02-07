use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<'a, T: 'a, K: 'a, V: 'a>
    where 
        T: Fn(&'a K) -> &'a V,
        K: Hash + Eq,
{
    calculation: T,
    values: HashMap<&'a K, &'a V>
}

impl<'a, T: 'a, K: 'a, V: 'a> Cacher<'a, T, K, V>
    where 
        T: Fn(&'a K) -> &'a V,
        K: Hash + Eq,
{
    fn new (calculation: T) -> Cacher<'a, T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: &'a K) -> &'a V {
        match self.values.get(arg) {
            Some(value) => value,
            None => {
                let value = &(self.calculation)(arg);
            }
        }
    }
}

fn main() {
    let mut expensive_result = Cacher::new(|num| {
        println!("caclculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    let v1 = expensive_result.value(1);
    let v2 = expensive_result.value(2);

    println!("Got value {} from the Cacher", v1);
    println!("Got value {} from the Cacher", v2);
}
