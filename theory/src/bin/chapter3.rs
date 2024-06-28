fn main() {
    /*
    Изменяемые и неизменяемые переменные. Константы
    */

    {
        // Неизменяемая переменная
        let x = 10;
        println!("x = {x}");
        // x = 5; // Выдаст ошибку.
        println!("x = {x}");
        let mut y = 10;
        println!("y = {y}");
        y = 5;
        println!("y = {y}");

        const PI: f32 = 3.14;
        println!("PI = {PI}");
        // Попытки присвоить константе другое значение или затенить её, приведут к ошибки.
        // let PI: f32 = 3.1415; // Ошибка
        // const PI: f32 = 3.1415; // Ошибка
        // PI = 3.1415; // Ошибка
    }

    /*
    Типы данных.
    */

    {
        let number: i32 = "41".parse().expect("not a number");
        println!("number = {number}");

        // Переполнение целых
        let mut x: u8 = 255;
        // x += 1; // Ошибка
        // Проверка на переполнение
        // x.checked_add(1).expect("overflow");
        x -= 1;
        println!("x = {x}");

        // Кортеж (классика)
        let tuple = (10, 20, 30);
        println!("tuple = {:?}", tuple);
        // Разыменовывание
        let (x, y, z) = tuple;
        println!("x = {x}, y = {y}, z = {z}");
        // Обращение к элементу кортежа
        println!("tuple[0] = {}", tuple.0);

        // Массив
        let mut array = [1, 2, 3, 4];
        println!("array = {:?}", array);
        println!("array[3] = {}", array[3]);
        let zeroes = [0; 10];
        println!("zeroes = {:?}", zeroes);
        // Выходить за пределы массива нельзя.
        // array[4] = 1; // Ошибка!
        // println!("array[4] = {}", array[4])
        array[3] = 4;
        println!("array[3] = {}", array[3]);
    }

    /*
    Операторы и выражения
    */

    {
        let x = {
            let y = 10;
            y
        };
        println!("x = {x}");
        // println!("y = {y}"); // Ошибка, так как y в другом пространстве.
    }
}
