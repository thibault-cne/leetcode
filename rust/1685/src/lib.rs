pub struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        ans[0] = nums.iter().sum::<i32>() - nums[0] * nums.len() as i32;

        for i in 1..nums.len() {
            ans[i] = ans[i - 1] + (nums[i] - nums[i - 1]) * (2 * i as i32 - nums.len() as i32);
        }

        ans
    }
}
