use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));

        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        let value = self.values.get(&arg);
        match value {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        // arrange
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        assert_eq!(v1, 1);

        // act
        let v2 = c.value(2);
        // assert
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_different_strings() {
        // arrange
        let mut c = Cacher::new(|a| a);

        let v1 = c.value("abc");
        assert_eq!(v1, "abc");

        // act
        let v2 = c.value("def");
        // assert
        assert_eq!(v2, "def");
    }

    #[test]
    fn call_with_different_type_values() {
        // arrange
        let mut c = Cacher::new(|a: &str| a.len());

        let v1 = c.value("abc");
        assert_eq!(v1, 3);

        // act
        let v2 = c.value("defghi");
        // assert
        assert_eq!(v2, 6);
    }
}
