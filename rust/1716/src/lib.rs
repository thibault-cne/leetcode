pub struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let q = n / 7;
        let r = n % 7;

        q * 28 + 7 * (q * (q - 1) / 2) + r * q + (r * (r + 1) / 2)
    }
}
