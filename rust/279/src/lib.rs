pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![i32::MIN; (n + 1) as usize];

        rec(n, &mut dp)
    }
}

fn rec(n: i32, dp: &mut [i32]) -> i32 {
    if n == 1 {
        return 1;
    }

    if dp[n as usize] != i32::MIN {
        return dp[n as usize];
    }

    let sqrt = (n as f64).sqrt() as i32;

    if sqrt * sqrt == n {
        return 1;
    }

    let mut res = i32::MAX;
    for i in 1..=sqrt {
        res = res.min(1 + rec(n - i * i, dp));
    }

    dp[n as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
