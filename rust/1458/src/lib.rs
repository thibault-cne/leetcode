pub struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![i32::MIN; nums2.len() + 1]; nums1.len() + 1];
        for (i, n1) in nums1.iter().enumerate() {
            for (j, n2) in nums2.iter().enumerate() {
                dp[i + 1][j + 1] = std::cmp::max(
                    std::cmp::max(dp[i][j + 1], dp[i + 1][j]),
                    std::cmp::max(0, dp[i][j]) + n1 * n2,
                );
            }
        }
        dp[nums1.len()][nums2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_dot_product() {
        assert_eq!(
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6]),
            18
        );
        assert_eq!(Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]), 21);
        assert_eq!(
            Solution::max_dot_product(vec![-1, -1], vec![1, 1]),
            -1
        );
    }
}