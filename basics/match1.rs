// match.rs 

#[derive(Debug)]
enum UsState {
    Michigan,
    Georgia,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State is {:?}", state);
            25
        }
    }
}

fn main() {
    let p = Coin::Penny;
    let v1 = value_in_cents(p);
    println!("Penny value: {}", v1);

    let q = Coin::Quarter(UsState::Michigan);
    let v2 = value_in_cents(q);
    println!("Quarter value: {}", v2);
}