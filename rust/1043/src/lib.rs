pub struct Solution;

impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];

        (0..(arr.len() + 1)).for_each(|i| {
            (1..=(k as usize)).filter(|j| i >= *j).for_each(|j| {
                dp[i] =
                    dp[i].max(dp[i - j] + arr[(i - j)..=(i - 1)].iter().max().unwrap() * j as i32);
            });
        });

        dp[arr.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 15, 7, 9, 2, 5, 10];
        let k = 3;
        assert_eq!(Solution::max_sum_after_partitioning(arr, k), 84);
    }
}
