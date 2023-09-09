pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];

        dp[0] = 1;

        (1..=target).for_each(|i| {
            nums.iter()
                .filter(|&j| i - j >= 0)
                .for_each(|&j| dp[i as usize] += dp[i as usize - j as usize]);
        });

        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let list = vec![1, 2, 3];

        assert_eq!(Solution::combination_sum4(list, 4), 7);
    }
}
