use std::{collections::HashMap, hash::Hash, thread, time::Duration};

pub fn run() {
    let simulated_user_specified_value = 19;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number)
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation: T,
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> V {
        let v = self.value.entry(arg).or_insert((self.calculation)(arg));
        *v
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let mut expensive_result = Cacher::new(simulated_expensive_calculation);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
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
    use super::Cacher;

    #[test]
    fn call_with_different_value() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    #[test]
    fn caputer_environment() {
        let x = 4;

        let equal_to_x = |z| z == x;

        let y = 4;

        assert!(equal_to_x(y));
    }

    #[test]
    fn caputre_environment_move() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;

        // println!("cant use x here {:?}", x);

        let y = vec![1, 2, 3];

        assert!(equal_to_x(y));
    }
}
