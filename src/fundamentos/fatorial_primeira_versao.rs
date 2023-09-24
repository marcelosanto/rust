use std::io;

fn main() {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error ao ler o numero 1");

    let mut x = convert_to_int(&number);
    let mut y = 1;

    while x > 0 {
        y = x * y;
        x -= 1;
    }

    println!("O Fatorial do {} é {}", number, y)
}

fn convert_to_int(data_input: &String) -> BigUint {
    let x = data_input.trim().parse::<BigUint>().unwrap();
    x
}
