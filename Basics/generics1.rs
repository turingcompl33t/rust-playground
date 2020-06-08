// generics1.rs

fn largest<T>(container : &[T]) -> T 
    where T : PartialOrd + Copy
{
    let mut l = container[0];
    for &item in container {
        if item > l {
            l = item;
        }
    }
    l
}

fn main() {
    let c1 = vec![1, 2, 3];
    let l1 = largest(&c1);
    println!("Largest is: {}", l1);
}