use std::io;

fn main() {
    let mut number_01 = String::new();

    io::stdin()
        .read_line(&mut number_01)
        .expect("Error ao ler o numero 1");

    let mut number_02 = String::new();

    io::stdin()
        .read_line(&mut number_02)
        .expect("Error ao ler o numero 2");

    let x = convert_to_int(&number_01);
    let y = convert_to_int(&number_02);

    if y > x {
        println!("{} é maior que {}", y, x)
    } else {
        println!("{} é maior que {}", x, y)
    }
}

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
