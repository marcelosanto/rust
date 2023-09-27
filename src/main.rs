fn media(numeros: &Vec<i32>) -> f64 {
    let mut soma = 0;

    for i in numeros {
        soma += i;
    }

    soma as f64 / numeros.len() as f64
}

fn mediana(numeros: Vec<i32>) -> f64 {
    3.0
}

fn moda(numeros: Vec<i32>) -> i32 {
    3
}

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];

    println!("Media: {}", media(&numeros));
}
