pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![i32::MIN; (target + 1) as usize]; (n + 1) as usize];

        rec(&mut dp, n, k, target)
    }
}

fn rec(dp: &mut Vec<Vec<i32>>, n: i32, k: i32, target: i32) -> i32 {
    if n == 0 && target == 0 {
        return 1;
    }
    if n == 0 || target <= 0 {
        return 0;
    }

    if dp[n as usize][target as usize] != i32::MIN {
        return dp[n as usize][target as usize];
    }

    let ways = (1..=k).fold(0, |acc, v| (acc + rec(dp, n - 1, k, target - v)) % MOD);

    dp[n as usize][target as usize] = ways;

    ways
}
