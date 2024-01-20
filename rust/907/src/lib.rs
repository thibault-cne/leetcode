pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut ans = arr[0];
        let mut stack = vec![(0, arr[0])];
        let mut dp = vec![0; arr.len()];

        dp[0] = arr[0];

        (1..arr.len()).for_each(|i| {
            let mut t = arr[i];

            while let Some(last) = stack.last() {
                if last.1 >= arr[i] {
                    stack.pop();
                } else {
                    break;
                }
            }

            if stack.is_empty() {
                t = (t + i as i32 * arr[i]) % MOD;
            } else {
                let j = stack.last().unwrap().0;

                t = (t + (i - j - 1) as i32 * arr[i]) % MOD;
                t = (t + dp[j]) % MOD;
            }

            dp[i] = t;
            ans = (ans + t) % MOD;

            stack.push((i, arr[i]));
        });

        ans
    }
}
