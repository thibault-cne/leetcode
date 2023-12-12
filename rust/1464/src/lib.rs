pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len();
        nums.sort_unstable();

        (nums[n - 2] - 1) * (nums[n - 1] - 1)
    }
}
