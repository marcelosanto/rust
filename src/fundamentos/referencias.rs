fn main() {
    let mut x = 32;
    let y = &mut x;
    //let z = y;

    *y += 15;

    println!("{}", x);
}
