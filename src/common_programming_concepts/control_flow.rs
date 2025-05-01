// во первых во все if/while мы всегда должны отдавать boolean,
// так как rust не умеет самостоятельно конвертировать значения
// if можено использовать для присваивания перменной, никакого ниндзя стайл нету в духе true ? 5 : 6
// есть loop, цикл без условий, ему можно давать кастомные имена, как понял для while тоже
// в rust только for-in, никакого for (init; condition; increment)
#[allow(dead_code)]
pub fn test() {
    let condition = true;
    let mut number = if condition { 5 } else { 6 };
    let vec: Vec<u32> = (0..=number).collect();

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 1 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count} \n");

    while number != 0 {
        println!("Number is {number}!");

        number -= 1;
    }

    println!("Number = {number} \n END \n");

    // нужно указывать, что мы ссылаемся на вектор, а не копируем его значения
    // иначе следующий цико ниже не будет работать
    for element in &vec {
        println!("the value is: {element}");
    }
    println!("\nDONE\n");

    for index in 0..vec.len() {
        println!("the value is: {}", vec[index]);
    }
}