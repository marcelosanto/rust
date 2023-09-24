use std::f32::consts::E;

struct Pessoa {
    name: String,
    idade: i32,
}

trait Voz {
    fn falar(&self);
    fn tem_voz(&self) -> bool;
}

impl Voz for Pessoa {
    fn falar(&self) {
        println!("Olá meu nome é {}", self.name)
    }

    fn tem_voz(&self) -> bool {
        if self.idade > 1 {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    let eu = Pessoa {
        name: String::from("Marcelo"),
        idade: 32,
    };

    eu.falar();
    println!("eu tenho voz: {}", eu.tem_voz());
}
