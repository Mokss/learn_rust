// сначала объявляем структуру, потом описываем ее методы
// пока я не понял есть классы тут и что с наследованием

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // первый аргумент, это всегда this(javascript)=self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[allow(dead_code)]
pub fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}