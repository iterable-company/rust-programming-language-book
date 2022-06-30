fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(0) => println!("zero"),
        _ => (),
    }

    if let Some(0) = some_u8_value {
        println!("zero 2");
    }

    let mut count1 = 0;
    let coins = [Coin::Penny, Coin::Dime, Coin::Quarter(UsState::Alabama)];
    for coin in coins.iter() {
        match coin {
            Coin::Quarter(state) => println!("state is {:#?}", state),
            _ => count1 += 1,
        }
    }

    let mut count2 = 0;
    for coin in coins.iter() {
        if let Coin::Quarter(state) = coin {
            println!("state2 is {:#?}", state);
        } else {
            count2 += 1;
        }
    }

    println!("count1 = {}, count2 = {}", count1, count2);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
