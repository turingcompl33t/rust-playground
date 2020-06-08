// closures1.rs

use std::thread;
use std::time::Duration;

struct Cacher<F> 
    where F : Fn(u32) -> u32
{
    closure : F,
    result: Option<u32>,
}

impl<F> Cacher<F>
    where F : Fn(u32) -> u32
{
    fn new(closure : F) -> Cacher<F> {
        Self {
            closure,
            result: None
        }
    }

    fn value(&mut self, arg : u32) -> u32 {
        match self.result {
            Some(r) => r,
            None => {
                let r = (self.closure)(arg);
                self.result = Some(r);
                r
            }
        }
    }
}

fn main() {
    let expensive_computation = |arg : u32| -> u32 {
        println!("Executing expensive computation...");
        thread::sleep(Duration::from_secs(3));
        println!("Done!");
        arg + 1
    };

    let mut cacher = Cacher::new(expensive_computation);

    let r1 = cacher.value(2);
    let r2 = cacher.value(2);

    println!("Result 1: {}", r1);
    println!("Result 2: {}", r2);
}