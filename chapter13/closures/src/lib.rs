use std::thread;
use std::time::Duration;
use std::collections::HashMap;

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::marker::Copy + std::cmp::Eq + std::hash::Hash,
{
    calculation: T,
    map: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: std::marker::Copy + std::cmp::Eq + std::hash::Hash,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> &V {
        #[allow(clippy::or_fun_call)]
        self.map.entry(arg).or_insert((self.calculation)(arg))
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        
        let v1 = c.value(1).clone();
        assert_eq!(v1, 1);
        let v1 = c.value(1).clone();
        assert_eq!(v1, 1);
        let v2 = c.value(2).clone();
        assert_eq!(v2, 2);
        let v2 = c.value(2).clone();
        assert_eq!(v2, 2);
    }
    
    #[test]
    fn call_with_different_closure() {
        let mut c = Cacher::new(|a| a + 1);
        
        let v1 = c.value(1);
        assert_eq!(*v1, 2);
    }
    
    #[test]
    fn call_with_different_closure2() {
        let mut c = Cacher::new(|a: u32| a.to_string());
        
        let v1 = c.value(1);
        assert_eq!(v1, "1");
    }
}
