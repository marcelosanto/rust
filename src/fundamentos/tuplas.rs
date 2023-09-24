struct Game(String, String, i32, bool);

fn main() {
    let jogo = Game(String::from("Starfield"), String::from("XBOX"), 2023, true);

    println!("O nome do jogo é {}", jogo.0);
    println!("A empresa que publicou é {}", jogo.1);
    println!("O ano de lançamento é {}", jogo.2);
    println!("O jogo é bom? {}", jogo.3);
}
