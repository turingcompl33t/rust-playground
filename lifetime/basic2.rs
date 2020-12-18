// basic2.rs

struct Excerpt<'a> {
    part: &'a str
}

fn main() {
    let book = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = book.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence
    };

    println!("First sentence: {}", excerpt.part);
}