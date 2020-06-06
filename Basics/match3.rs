// match3.rs

fn do_match(n : Option<i32>) {
    if let Some(5) = n {
        println!("Got Some(5)");
    } else {
        println!("Did not get Some(5)");
    }
}

fn main() {
    let n1 = Some(5);
    let n2 = Some(6);

    do_match(n1);
    do_match(n2);
}