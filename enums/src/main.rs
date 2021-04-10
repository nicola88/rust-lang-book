#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn main() {
    let ip4 = IpAddrKind::V4(12,12,45,67);
    let ip6 = IpAddrKind::V6(String::from("::1"));
    println!("ip4 = {:?}", ip4);
    println!("ip6 = {:?}", ip6);

    println!("1 penny = {} cent(s)", value_in_cents(Coin::Penny));
    println!("1 nickel = {} cent(s)", value_in_cents(Coin::Nickel));
    println!("1 dime = {} cent(s)", value_in_cents(Coin::Dime));
    println!("1 quarter = {} cent(s)", value_in_cents(Coin::Quarter(UsState::Alabama)));
}
