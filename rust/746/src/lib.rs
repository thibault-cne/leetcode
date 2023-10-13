pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len()];

        for i in (0..cost.len()).rev() {
            if i == cost.len() - 1 || i == cost.len() - 2 {
                dp[i] = cost[i];
            } else {
                dp[i] = cost[i] + std::cmp::min(dp[i + 1], dp[i + 2]);
            }
        }

        std::cmp::min(dp[0], dp[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
