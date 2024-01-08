pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = vec![std::collections::HashMap::new(); nums.len()];

        (1..nums.len()).for_each(|i| {
            (0..i).for_each(|j| {
                let diff = nums[i] as i64 - nums[j] as i64;
                *dp[i].entry(diff).or_insert(0) += 1;

                if let Some(&v) = dp[j].get(&diff) {
                    *dp[i].entry(diff).or_insert(0) += v;
                    ans += v;
                }
            });
        });

        ans
    }
}
