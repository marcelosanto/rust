enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}

fn main() {
    let player: Direction = Direction::Up;
    let player_male: Gender = Gender::Male;
    let player_female: Gender = Gender::Female;

    match player {
        Direction::Up => println!("O Jogador foi para cima"),
        Direction::Down => println!("O Jogador foi para baixo"),
        Direction::Left => println!("O Jogador foi para esquerda"),
        Direction::Right => println!("O Jogador foi para direita"),
    }

    println!("{:?} é casado com {:?}", player_male, player_female);
}
