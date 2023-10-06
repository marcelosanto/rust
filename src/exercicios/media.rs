use std::io;

fn media() {
    let mut nota_a = String::new();
    let mut nota_b = String::new();
    let mut nota_c = String::new();

    io::stdin()
        .read_line(&mut nota_a)
        .expect("Erro ao ler o primeiro número");

    io::stdin()
        .read_line(&mut nota_b)
        .expect("Erro ao ler o primeiro número");

    io::stdin()
        .read_line(&mut nota_c)
        .expect("Erro ao ler o primeiro número");

    let nota_a: f32 = nota_a.trim().parse().expect("Insira numero");
    let nota_b: f32 = nota_b.trim().parse().expect("Insira numero");
    let nota_c: f32 = nota_c.trim().parse().expect("Insira numero");

    let media = ((nota_a * 2.0) + (nota_b * 3.0) + (nota_c * 5.0)) / 10.0;

    println!("MEDIA = {:.1}", media);
}
