use std::io;
// в целом типы все как у всех со строгой типизацией, массивы фиксированной длинны и строго одного типа
// примитивы и в африке примитивы
// есть уникальный тип usize, который как бы говорит я число, но какой именно не знаю
// можно коротко описать тип let a: [i32; 5] = [1, 2, 3, 4, 5];
// или сразу инициализировать значения let a = [3; 5];

#[allow(dead_code)]
pub fn data_types() {
    // что то типо деструктуризации в js
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x} and {y} and {z}");
    println!("{} and {} and {}", tup.0, tup.1, tup.2);


    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // мега прикольно что выход из цикла можно записывать в переменные
    let index: usize = loop {
        let mut index_str = String::new();

        io::stdin()
            .read_line(&mut index_str)
            .expect("Failed to read line");

        match index_str.trim().parse() {
            Ok(num) => break num, // выход из цикла с результатом
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        }
    };

    // естественно если индекс больше размера массива, будет ошибка в рантайме.
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}