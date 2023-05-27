fn main() {
    enum_();
    match_();
    if_let();
}

fn enum_() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));

    let home2 = IpAddr2::V4(127, 0, 0, 1);
    let loopback2 = IpAddr2::V6(String::from("::1"));
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr3 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn match_() {
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
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn if_let() {
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
