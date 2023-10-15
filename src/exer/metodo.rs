trait Acao {
    fn falar(&self);
    fn idade(&self) -> i32;
}

pub struct Pessoa {
    pub nome: String,
    pub sobrenome: String,
    pub idade: i32,
}

impl Pessoa {
    pub fn falar(&self) {
        println!("Nome completo: {} {}", self.nome, self.sobrenome)
    }
}
