// higher_order1.rs

fn basic_map<F>(v: &Vec<i32>, func: F) -> Vec<i32>
    where F: Fn(i32) -> i32
{
    let mut c: Vec<i32> = Vec::new();
    for item in v {
        c.push(func(*item));
    }
    c
}

fn main() {
    let v = vec![1, 2, 3];
    let f = |n : i32| -> i32 { n + 1 };

    let r = basic_map(&v, f);

    println!("{:?}", v);
    println!("{:?}", r);
}