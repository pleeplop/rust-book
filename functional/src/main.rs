use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use core::hash::Hash;

fn main() {
    closure_environment();
}

fn closure_environment() {
    let x = 3;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("{:?}", (equal_to_x)(y));

    let v = vec![1, 2, 3];
    let v_plus_one: Vec<i32> = v.iter().map(|x| x + 1).collect();
    println!("{:?}", v_plus_one);

}

fn workout() {
    let simulated_user_specified_user_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_user_value,
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today do {} pushups!", expensive_closure.value(intensity));
        println!("Next do {} situps", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break! Remember to stay hydrated");
        } else {
            println!("Today run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

struct Cacher<T,K,V>
    where T: Fn(K) -> V {
    calculation: T,
    map: HashMap<K, V>,
}

impl<T,K,V> Cacher<T,K,V>
    where T: Fn(K) -> V,
          K: Eq + Hash + Copy
{

    fn new(calculation: T) -> Cacher<T,K,V> {
        Cacher { calculation, map: HashMap::new() }
    }

    fn value(&mut self, arg: K) -> &V {
        let calculation = &self.calculation;
        let x = self.map.entry(arg)
            .or_insert_with(|| (*calculation)(arg));
        x
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly..");
    thread::sleep(Duration::from_secs(2));
    intensity
}