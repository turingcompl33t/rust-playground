// while.rs

fn main() {
    let mut counter : i32 = 3;
    while counter >= 0 {
        println!("Counter = {}", counter);
        counter -= 1;
    }
    println!("Done");
}