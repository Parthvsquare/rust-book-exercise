// enum IpAddress{
//     V4(String),
//     V6(String)
// }

#[derive(Debug)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Messages {
    fn call(&self) {
        println!("the created ipv4 is {:?}", self);
    }
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // let ipV4 = IpAddress::V4(String::from("1.1.1.3"));
    // println!("the created ipv4 is {:?}", ipV4);
    let m = Messages::Write(String::from("Hello"));
    m.call();

    let x = Option::Some(5);
    let six = plus_something(x);
    // let absent_number: Option<i32> = Option::None;
    println!("the sum is {:?}", six);
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Luck penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_something(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
        Option::None => Option::None,
    }
}
