pub struct Solution;

impl Solution {
    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, n) in nums.iter().enumerate() {
            for (j, m) in nums.iter().skip(i + 1).enumerate() {
                if n + m == target {
                    return vec![i as i32, (j + 1 + i) as i32];
                }
            }
        }

        vec![]
    }
}
