struct Gamer {
    username: String,
    email: String,
    active: bool,
    gender: String,
}

impl Gamer {
    fn name_of_user(&self) {
        println!("O nome do usuario é {}", self.username);
    }

    fn user_active(&self) {
        println!("O usuario esta ativo? {}", self.active);
    }
}

fn main() {
    let user = Gamer {
        username: String::from("Marcelo"),
        email: String::from("lols@lols.com"),
        active: true,
        gender: String::from("Homem"),
    };

    user.name_of_user();
    user.user_active();
}
