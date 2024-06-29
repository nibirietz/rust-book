use rand::Rng;
use std::io;

fn main() {
    let guess_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut user_number = String::new();
        io::stdin()
            .read_line(&mut user_number)
            .expect("Неправильный ввод.");
        let user_number: i32 = match user_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Неправильный ввод, попробуйте снова.");
                continue;
            }
        };
        if user_number > guess_number {
            println!("Слишком большое!");
        } else if user_number == guess_number {
            println!("Вы правильно отгадали число!");
            break;
        } else {
            println!("Слишком маленькое!");
        }
    }
}
