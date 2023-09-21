fn main() {
    let x = [1, 33, 55, 66, 99, 9, 2, 8, 10, 444];
    let _f = 3.6;
    let _b = true;

    for i in x {
        if i % 2 == 0 {
            println!("{} é numero par", i)
        } else {
            println!("{} é numero impar", i)
        }
    }
}
