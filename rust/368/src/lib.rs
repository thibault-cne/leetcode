pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut dp = vec![Vec::new(); nums.len()];

        let mut r = 0;
        let mut v = Vec::new();

        (0..nums.len()).for_each(|i| {
            search(i, &nums, &mut dp);
            if r < dp[i].len() {
                r = dp[i].len();
                v = dp[i].clone();
            }
        });

        v.reverse();

        v
    }
}

fn search(i: usize, nums: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> Vec<i32> {
    if !dp[i].is_empty() {
        dp[i].clone()
    } else {
        let mut r = 0;
        let mut v = Vec::new();

        (i + 1..nums.len())
            .filter(|&j| nums[j] % nums[i] == 0 || nums[i] % nums[j] == 0)
            .for_each(|j| {
                let w = search(j, nums, dp);

                if r < w.len() {
                    r = w.len();
                    v = w;
                }
            });

        v.push(nums[i]);

        dp[i] = v.clone();

        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 3]),
            vec![1, 2]
        );
        assert_eq!(
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
            vec![1, 2, 4, 8]
        );
    }
}
