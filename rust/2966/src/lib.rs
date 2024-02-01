pub struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; 3]; nums.len() / 3];
        nums.sort_unstable();

        for (i, e) in ans.iter_mut().enumerate() {
            let j = i * 3;

            if nums[j + 2] - nums[j] > k {
                return vec![];
            } else {
                *e = vec![nums[j], nums[j + 1], nums[j + 2]];
            }
        }

        ans
    }
}
