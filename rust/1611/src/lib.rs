pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut ans = 0;

        for shift in 0..32 {
            if n >> shift & 1 == 1 {
                ans = (1 << (shift + 1)) - 1 - ans;
            }
        }

        ans
    }
}
