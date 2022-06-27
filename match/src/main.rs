fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let price = value_in_cents(coin);
    println!("price is {}", price);

    let some_u8_value = 0u8;
    match some_u8_value {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        4 => println!("four"),
        8 => println!("eight"),
        _ => println!("other"),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state is {:#?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}