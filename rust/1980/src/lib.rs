pub struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut ans = Vec::with_capacity(nums.len());

        for (i, v) in nums.iter().enumerate() {
            match v.as_bytes()[i] {
                b'1' => ans.push('0'),
                b'0' => ans.push('1'),
                _ => unreachable!(),
            }
        }

        ans.into_iter().collect()
    }
}
