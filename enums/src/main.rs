
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self) {

    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let write = Message::Write(String::from("Hello, World!"));
    write.call();
    let some_str = Some(String::from("hello"));
    let none: Option<String> = None;

    let some_u8_value = 0u8;
    match some_u8_value {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    };

    println!("{:?}", 1);

    let s3 = Some(3);
    if let Some(3) = s3 {
        println!("some 3");
    }

    let mut count = 0;
    let pen = Coin::Penny;
    let alaska = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(usState) = alaska {
        println!("Us state quarter from {:?}", usState);
    } else {
        count += 1;
    }
    println!("count : {}", count);
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?} state", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(n) => Some(n+1),
        None => None,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}