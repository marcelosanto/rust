fn main() {
    let mut minhaString: String = String::from("Oieh meu nome é Marcelo.");

    println!("O tamanho dessa string é: {}", minhaString.len());

    println!("O tamanho dessa string é: {}", minhaString.is_empty());

    for token in minhaString.split_whitespace() {
        println!("{}", token)
    }

    println!(
        "O Nome Marcelo esta contido na String? {}",
        minhaString.contains("Marcelo")
    );

    minhaString.push_str(" E sou um programador");
    println!("{}", minhaString)
}
