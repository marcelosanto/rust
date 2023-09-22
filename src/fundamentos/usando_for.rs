fn main() {
    let animais = vec!["cachorro", "gato", "periquito", "coelho"];

    for a in animais {
        println!("Eu tenho o {} de animal", a.to_ascii_uppercase())
    }
}
