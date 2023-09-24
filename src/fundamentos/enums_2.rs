#[derive(Debug)]
enum CarType {
    Fiat,
    Ford,
    Renault,
}

fn nacionalidade_carro(car: CarType) {
    match car {
        CarType::Fiat => println!("O carro é Italiano."),
        CarType::Ford => println!("O carro é Americano."),
        CarType::Renault => println!("O carro é Francês."),
    }
}

fn main() {
    nacionalidade_carro(CarType::Ford);
    nacionalidade_carro(CarType::Fiat);
    nacionalidade_carro(CarType::Renault);
}
