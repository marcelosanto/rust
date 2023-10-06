use std::io;

fn main() {
    let mut numero = String::new();

    io::stdin()
        .read_line(&mut numero)
        .expect("Erro ao ler o primeiro número");

    let numero: Vec<&str> = numero.split(" ").collect();
    let numero_a: f32 = numero[0].trim().parse().expect("Insira numero");
    let numero_b: f32 = numero[1].trim().parse().expect("Insira numero");
    let numero_c: f32 = numero[2].trim().parse().expect("Insira numero");

    let area_triangulo = (numero_a * numero_c) / 2.0;
    let area_circulo = (numero_c.powf(2.0)) * (std::f32::consts::PI);
    let area_trapezio = ((numero_a + numero_b) * numero_c) / 2.0;
    let area_quadrado = numero_b * numero_b;
    let area_retangulo = numero_a * numero_b;

    println!("Area do triangulo: {:.3}", area_triangulo.round());
    println!("Area do circulo: {:.3}", area_circulo.round());
    println!("Area do trapezio: {:.3}", area_trapezio.round());
    println!("Area do quadrado: {:.3}", area_quadrado.round());
    println!("Area do retangulo: {:.3}", area_retangulo.round());
}
