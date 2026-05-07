fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    let four_call = route(four);
    let six_call = route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddressKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddressKind) {}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//enumはデータとしては別だけど、属性・ドメインが同じものに対して効果的(例：IPアドレス。)
enum IpAddrs {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn call_ip_addrs() {
    let home = IpAddrs::V4(127, 0, 0, 1);
    let loopback = IpAddrs::V6(String::from("::1"));
}

//こんな感じのものもいける
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//ジェネリクスもありまっせ。でもジェネリクスを直接足し算するのは無理やで
fn generics() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// pennyだったら1を割り当てるみたいなそんなイメージ。switch文が近い
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        other => 99, //finally defaultみたいなことができる
    }
}

fn not_use() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), //値を定義しない的なこともできる。なのでこの場合だと3か7以外無効票
                       //_ => (), こんな書き方もできる。reroll()は不要
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn if_match() {
    //matchを直感的に書いた方法。ただし列挙的に書けないのがデメリット
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
