use std::collections::HashMap;

fn main() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: HashMap<u32, u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: HashMap::new(),
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value.get(&arg) {
                Some(&v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value.insert(arg, v);
                    v
                }
            }
        }
    }

    let mut c = Cacher::new(|x| x);

    println!("{} {}", c.value(1), c.value(2));
    println!("{} {}", c.value(1), c.value(3))
}
