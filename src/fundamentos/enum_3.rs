enum Pagamentos {
    Dinheiro(f32),
    CartaoCredito(bool, f32),
}

fn main() {
    let pessoa_pagamentos = Pagamentos::CartaoCredito(false, 100f32);

    match pessoa_pagamentos {
        Pagamentos::Dinheiro(f) => println!("A pessoa pagou {} reais no Dinheiro", f),
        Pagamentos::CartaoCredito(true, x) => {
            println!("A pessoa pagou {} reais no Cartao de Credito", x)
        }
        Pagamentos::CartaoCredito(false, _) => {
            println!("Pagamento no Cartao de Credito não passou",)
        }
        _ => {}
    }
}
