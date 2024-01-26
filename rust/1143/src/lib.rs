pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        (0..len1).rev().for_each(|i| {
            (0..len2).rev().for_each(|j| {
                if text1.as_bytes()[i] == text2.as_bytes()[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            });
        });

        dp[0][0]
    }
}
