fn media(numeros: &Vec<i32>) -> f64 {
    let mut soma = 0;

    for i in numeros {
        soma += i;
    }

    soma as f64 / numeros.len() as f64
}

fn mediana(numeros: &Vec<i32>) -> f64 {
    let mut numeros_sorted = numeros.clone();
    numeros_sorted.sort();

    let numero_meio = numeros.len() / 2;
    if numeros_sorted.len() % 2 == 0 {
        return media(&vec![
            numeros_sorted[numero_meio - 1],
            numeros_sorted[numero_meio],
        ]) as f64;
    }

    return numeros_sorted[numero_meio] as f64;
}

fn moda(_numeros: Vec<i32>) -> i32 {
    3
}

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];

    println!("Media: {}", media(&numeros));
    println!("Mediaana: {}", mediana(&numeros));
}
