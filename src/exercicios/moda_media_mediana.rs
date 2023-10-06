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

fn moda(numeros: &Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();

    for i in numeros {
        let contar = map.entry(i).or_insert(0);

        *contar += 1;
    }

    let mut maior_valor = 0;
    let mut maior_key = 0;

    for (key, value) in map {
        if value > maior_valor {
            maior_valor = value;
            maior_key = *key;
        }
    }

    maior_key
}

fn main() {
    let numeros = vec![1, 1, 2, 3, 4, 4, 4, 5, 6, 0];

    println!("Media: {}", media(&numeros));
    println!("Mediana: {}", mediana(&numeros));
    println!("Moda: {}", moda(&numeros));
}
