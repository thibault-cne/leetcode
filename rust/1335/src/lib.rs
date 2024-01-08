pub struct Solution;

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        // Edge case
        match job_difficulty.len() {
            j if j < d as usize => return -1,
            j if j == d as usize => return job_difficulty.iter().sum(),
            _ => (),
        }

        let mut dp = vec![vec![0; job_difficulty.len()]; d as usize];

        dp[0][0] = job_difficulty[0];
        (1..job_difficulty.len()).for_each(|j| dp[0][j] = dp[0][j - 1].max(job_difficulty[j]));

        (1..(d as usize)).for_each(|day| {
            (day..job_difficulty.len()).for_each(|i| {
                let mut local_max = job_difficulty[i];
                dp[day][i] = i32::MAX;

                (day..=i).rev().for_each(|j| {
                    local_max = local_max.max(job_difficulty[j]);
                    dp[day][i] = dp[day][i].min(dp[day - 1][j - 1] + local_max);
                });
            })
        });

        *dp.last().unwrap().last().unwrap()
    }
}
