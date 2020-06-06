// for.rs

fn main() {
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    for n in arr.iter() {
        println!("n = {}", n);
    }
}