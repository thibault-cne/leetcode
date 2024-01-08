pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut dp = vec![0; start_time.len() + 1];
        let mut v: Vec<_> = start_time
            .into_iter()
            .zip(end_time)
            .zip(profit)
            .map(|((s, e), p)| (e, s, p))
            .collect();

        v.sort_unstable();

        v.iter().enumerate().for_each(|(i, &(_, s, p))| {
            let j = v[..i].partition_point(|&(e, _, _)| e <= s);
            dp[i + 1] = dp[i].max(dp[j] + p);
        });

        *dp.last().unwrap()
    }
}
