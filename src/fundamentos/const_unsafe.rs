static mut STATIC_VARIAVEL: f32 = 3.14;

fn main() {
    let a: f32 = 10.1;

    unsafe { println!("A Soma é {}", a * STATIC_VARIAVEL) }
}
