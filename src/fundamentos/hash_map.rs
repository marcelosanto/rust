use std::collections::HashMap;

fn main() {
    let mut hash_map = HashMap::new();

    hash_map.insert(0, "Marcelo");
    hash_map.insert(1, "Alice");
    hash_map.insert(2, "Maquerle");
    hash_map.insert(3, "Gabriel");
    hash_map.insert(4, "Luna");
    hash_map.insert(5, "Livia");

    println!("Quantos dados tem no hash map? {}", hash_map.len());

    hash_map.remove(&0);

    match hash_map.get(&0) {
        Some(x) => println!("O Nome do pai é: {}", x),
        None => println!("Nome do pai não encontrado."),
    }
}
