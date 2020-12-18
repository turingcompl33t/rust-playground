// ownership1.rs

fn makes_copy(n : i32) {
    println!("n = {}", n);
}

fn takes_ownership(s : String) {
    println!("s = {}", s);
} // s dropped here

fn main() {
    let n = 42;
    let s = String::from("42");

    makes_copy(n);
    takes_ownership(s);

    // s is invalid here
}