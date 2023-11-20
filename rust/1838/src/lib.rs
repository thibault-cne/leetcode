pub struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, mut k: i32) -> i32 {
        let mut ans = 1;
        nums.sort_unstable();

        let mut i = 0;

        for j in 1..nums.len() {
            k -= (nums[j] - nums[j - 1]) * (j - i) as i32;

            while k < 0 {
                k += nums[j] - nums[i];
                i += 1;
            }

            ans = ans.max(j - i + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 4];
        let k = 5;
        let ans = 3;
        assert_eq!(Solution::max_frequency(nums, k), ans);
    }
}
