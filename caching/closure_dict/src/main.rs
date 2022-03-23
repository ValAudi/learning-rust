use std::hash::Hash;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;


struct Cacher<T, K, V>
where T:Fn(K) -> V,
      K: Eq + Hash + Copy
{
    closure_func: T,
    value: HashMap<K, V>,
}

impl <T, K, V> Cacher<T, K, V>
where T:Fn(K) -> V,
      K: Eq + Hash + Copy
{
    fn new(closure_func:T)-> Cacher<T, K, V>{
        Cacher { 
            closure_func, 
            value: HashMap::new(),
        }
    }
    fn value(&mut self, param: &K) -> &V {
        // 
        if self.value.contains_key(param) {
           let val: &V = self.value.get(param).unwrap();
           val
        }
        else {
            let v = (self.closure_func)(*param);
            self.value.entry(*param).or_insert(v);
            self.value.get(param).unwrap()
        }
    
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result= Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(&intensity));
        println!("Next, do {} situps!", expensive_result.value(&intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",expensive_result.value(&intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 40; 
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
