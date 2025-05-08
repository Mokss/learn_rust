// раст имеет такие же структуры как и в других языках
// сначала описываешь типы, потом присваиваешь их и в качесте типа переменной казываешь структуру
// там есть прикол с указателями, если ты хочешь чтобы свойство структуры являлось указателем на что либо
// то нужно ей явно говорить, что данные, на которые ссылается структура, будут действительны, пока живет сама структура
// в целом валидно, но пока все же немного сложно с пониманием и осознанием

// нужно добалять этот флаг для возможности выводить в console структуры
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
pub fn instance_struct() {
    // пример простой структуры, без указателей с одним владелцем для свойств
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        sign_in_count: 1,
    };
    // как и в js том же, можно спокойно обращатсья к свойствам через . и изменять его
    user1.username = String::from("update_username");
    user1.sign_in_count = 100;
    user1.active = false;
    println!("{:?}", user1);

    let user2 = build_user("mokss".to_owned());
    println!("{:?}", user2);

    // так же можно юзать спреад, как это реализовано в js
    let user3 = User {
        username: String::from("mokss2"),
        ..user1
    };
    println!("{:?}", user3);

    // можно просто как с кортежом писать
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Point(100, 1000, 100000);
    println!("{:?}", black);


    // ну и остались указатели
    #[derive(Debug)]
    struct Ponter<'a> {
        username: &'a str,
    }
    let name = String::from("MOKSS");
    let moks_user_name: Ponter<'_> = Ponter {
        username: &name,
    };
    println!("{:?}", moks_user_name);
}

fn build_user(username: String) -> User {
    return User {
        active: true,
        // можно как и в js сразу вставлять переменную, если имя свойства у структуры такое же
        username,
        sign_in_count: 1,
    };
}