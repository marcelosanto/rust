use std::io;

fn main() {
    let mut mensagem_usuario = String::new();

    println!("Hey boy, digite uma mensagem: ");

    match io::stdin().read_line(&mut mensagem_usuario) {
        Ok(_) => println!(
            "Sucesso voce digitou, {}",
            capitalize_first_letter(&mensagem_usuario)
        ),
        Err(e) => println!("Error: {}.", e),
    }
}

fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}
