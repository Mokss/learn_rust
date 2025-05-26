use std::borrow::Cow;

#[derive(Debug)] // so we can inspect the state in a minute
#[allow(dead_code)]
enum RusState {
    Alabama,
    Alaska,
    KrasnodarKrai,
}

impl RusState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            RusState::Alabama => year >= 1819,
            RusState::Alaska => year >= 1959,
            RusState:: KrasnodarKrai => year >= 1900,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(RusState),
}

const COINS: [Coin; 4]  = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter(RusState::KrasnodarKrai)];

// интересный код, по факту каждый раз мы будем возвращать указатели на строки
// в случае с константными строками, память не будет дополнительно выделятся
#[allow(dead_code)]
fn describe_number(n: u32) -> Cow<'static, str> {
    match n {
        // выгляди странно, но приколдесно
        1..=9 => "до десяти".into(),
        10..=19 => "до двадцати".into(),
        20..=29 => "до тридцати".into(),
        30..=39 => "до сорока".into(),
        40..=49 => "до пятидесяти".into(),
        50..=59 => "до шестидесяти".into(),
        60..=69 => "до семидесяти".into(),
        70..=79 => "до восьмидесяти".into(),
        80..=89 => "до девяноста".into(),
        90..=99 => "до ста".into(),
        // но в этом случае, на каждый other будет создаваться новый владелц и от него указатель на строку
        // раст все так же очиститься все строки и память, когда мы покинем область видимости, где вызовем describe_number
        other => format!("вне диапазона ({})", other).into(),
    }
}

#[allow(dead_code)]
fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            return  1;
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            return  25;
        }
    }
}
#[allow(dead_code)]
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

pub fn test() {
    for n in [3, 15, 27, 91, 105] {
        println!("{n} → {}", describe_number(n));
    }

    println!("\nspace\n");

    for n in  &COINS {
        println!("{n:?} → {}",  value_in_cents(n));
    }

    let str_dime = describe_state_quarter(Coin::Dime);
    let str_alabama = describe_state_quarter(Coin::Quarter(RusState::Alabama));
    let str_alaska_cry = describe_state_quarter(Coin::Quarter(RusState::Alaska));


    println!("{str_dime:?} {str_alabama:?} {str_alaska_cry:?}");

    return; 
}