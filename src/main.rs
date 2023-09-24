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

fn main() {}
