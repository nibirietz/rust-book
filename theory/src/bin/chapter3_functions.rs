use std::io;

fn main() {
    let x: i32 = custom_input().trim().parse().expect("Неправильный ввод.");
    let y: i32 = custom_input().trim().parse().expect("Неправильный ввод.");
    println!("max({x}, {y}) = {}", max(x, y))
}

/// Функция ввода, которая с stdin берёт символы и возвращает String
fn custom_input() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    return string;
}

/// Возвращает максимальный из двух i32.
fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}
