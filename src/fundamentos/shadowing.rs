fn main() {
    let a: i32 = 10;

    {
        println!("o valor de a e {}", a);
        let a: f32 = 20.309;
        println!("O valor de a eh {}", a)
    }

    println!("o valor de a e {}", a);
}
