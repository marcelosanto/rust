fn bubble_sort() {
    let mut array: Vec<i32> = vec![88, 55, 1, 3, 9, 8, -9, 0, -1, -5];

    println!("{:?}", array);

    for i in 0..array.len() {
        for j in ((i + 1)..array.len()).rev() {
            if array[j - 1] > array[j] {
                swap_array(&mut array, j - 1, j)
            }
        }
    }

    println!("{:?}", array);
}

fn swap_array(lista: &mut [i32], i: usize, j: usize) {
    let temp = lista[i];
    lista[i] = lista[j];
    lista[j] = temp;
}
