#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    KrasnodarskyCry,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn describe_number(n: u32) -> &'static str {
    match n {
        // выгляди странно, но приколдесно
        1..=9 => "до десяти",
        10..=19 => "до двадцати",
        20..=29 => "до тридцати",
        30..=39 => "до сорока",
        40..=49 => "до пятидесяти",
        50..=59 => "до шестидесяти",
        60..=69 => "до семидесяти",
        70..=79 => "до восьмидесяти",
        80..=89 => "до девяноста",
        90..=99 => "до ста",
        _ => "вне диапазона",
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

pub fn test() {
    for n in [3, 15, 27, 91, 105] {
        println!("{n} → {}", describe_number(n));
    }
}