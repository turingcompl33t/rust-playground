// basic1.rs

fn longest<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("abcd");
    let y = String::from("123");

    let r = longest(x.as_str(), y.as_str());
    println!("{} is longest", r);
}