// ownership2.rs

fn queries(s : &String) -> usize {
    s.len()
}

fn mutates(s : &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn main() {
    let s1 = String::from("hello");
    let l1 = queries(&s1);

    println!("string: {} length: {}", s1, l1);

    let mut s2 = String::from("hello");
    let l2 = mutates(&mut s2);

    println!("string: {} length: {}", s2, l2);
}