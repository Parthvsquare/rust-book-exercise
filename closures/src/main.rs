use std::{thread, time::Duration};

use closures::NewCacher;

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
    fn new(calculation: T) -> Cacher<T> {
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
fn main() {
    println!("Hello, world!");
    generate_workout(10, 3)
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!(
                "Today take a break for {} minutes",
                expensive_result.value(intensity)
            );
        } else {
            println!(
                "Today run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }
}

// problem with our closer method, we should have used hash map
#[test]
fn call_with_different_value() {
    let mut c = NewCacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
