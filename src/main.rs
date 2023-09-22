use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error ao ler o numero 1");

    let mut x = convert_to_int(&number);
    let mut soma = 0;

    while x != 0 {
        let r = x % 10;
        soma += r;
        x = x / 10;
    }

    println!("O resultado da soma dos digitos é {}", soma)
}

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
