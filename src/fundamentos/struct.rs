enum Gender {
    Male,
    Female,
}

struct User {
    username: String,
    email: String,
    active: bool,
    gender: Gender,
}

fn main() {
    let usuario = User {
        username: String::from("Marcelo"),
        email: "marcelo@lols.com".to_string(),
        active: true,
        gender: Gender::Male,
    };

    println!("Usuario: {}", usuario.username);
    println!("Email: {}", usuario.email);
    println!("Ativo: {}", usuario.active);

    match usuario.gender {
        Gender::Male => println!("Genero: Homem"),
        Gender::Female => println!("Genero: Mulher"),
    }
}
