// а вот это не прикольно, rust не имеет встроенной либы для получения рандомных чисел
// какая то + депенденси из интернета, ну вот зачем оно надо?
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("Guess the number!");

    // интересно выглядит 1..=100, но мне конечно больше нравится СИ подобный стайл, лучше бы просто 2 аргумента получал
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess.");

        // тема когда явно указывает что переменная мутабельная норм, но в тоже время, господи скока доп кода писать
        let mut guess = String::new();

        io::stdin()
            //  каайф ссылки
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // прикольно что можно сразу вызывать match и обробатывать удобно ошибки
            // например тут, мы просто заново перезапускаем цико, пока parse нам не вернет OK
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}