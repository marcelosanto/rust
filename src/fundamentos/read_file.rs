use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut arquivo = File::open("texto.txt").expect("Arquivo não encontrado");

    let mut conteudo = String::new();

    arquivo
        .read_to_string(&mut conteudo)
        .expect("Erro ao tentar ler arquivo e alocar na variavel conteudo");

    println!("O conteudo do arquivo é:\n\n {}", conteudo);
}
