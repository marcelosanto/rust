mod solution;

fn main() {
    let slt = solution::Solution::two_sums([21, 2, 7, 15].to_vec(), 9);
    println!("Solution: {:?}", slt);
}
