use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn simulated_expensive_caluculation(intensity: u32) -> u32 {
//     println!("calculation slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_caluculation(intensity);
    // let expensive_closure = |num| {
    //     println!("calculation slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    let mut cacher = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        // println!("Today, do {} pushups!", expensive_closure(intensity));

        // println!("Next, do {} situps!", expensive_closure(intensity));
        println!("Today, do {} pushups!", cacher.value(intensity));

        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            // println!("Today, run for {} minutes!", expensive_closure(intensity));
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
