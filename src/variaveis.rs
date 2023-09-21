fn variaveis() {
    //variaveis em rust são imutaveis
    //Shadowing
    let name = "Marcelo";
    let name = name.len();

    //Mutable
    let mut idade = 0;
    idade = 32;

    println!("Hello {}!", name);
    println!("Minha idade é {}!", idade);
}
