pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = std::collections::HashMap::new();

        Solution::climb_stairs_memoization(n, &mut dp)
    }

    fn climb_stairs_memoization(n: i32, dp: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if let Some(&res) = dp.get(&n) {
            return res;
        }

        let res = match n {
            0..=2 => n,
            n => {
                Solution::climb_stairs_memoization(n - 1, dp)
                    + Solution::climb_stairs_memoization(n - 2, dp)
            }
        };

        dp.insert(n, res);

        res
    }
}
