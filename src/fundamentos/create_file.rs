use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut arquivo = File::create("create.txt").expect("Erro ao criar o arquivo");

    arquivo
        .write_all(b"Testando criar o arquivo dessa maneira")
        .expect("erro ao tentar escrever no arquivo");
}
