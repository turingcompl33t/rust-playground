// match2.rs

fn plus_one(n : Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}