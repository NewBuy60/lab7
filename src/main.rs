use std::io;

fn main() {
    println!("Введите координаты!");
    let mut point = String::new();
    io::stdin()
            .read_line(&mut point)
            .expect("Failed to read line");

    let mut iter = point.split_ascii_whitespace();
    let mut arr:[i32;2] = [0, 0];
    let mut i = 0;
    loop {
        arr[i] = match iter.next() {
            Some(str) => match str.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Не верный формат координат!");
                    break;
                },
            },
            None => (
                if i < 1 {
                    println!("Не верный формат координат!");
                    break;
                } else {
                    break;
                }
            ),
        };
        println!("{}", arr[i]);
        i = i + 1;
    }   
    println!("x = {}, y = {}", arr[0], arr[1]);
    //println!("Point = {}",point);
}
