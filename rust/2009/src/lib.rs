pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        nums.sort_unstable();
        nums.dedup();

        nums.iter().enumerate().map(|(i, v)| {
            let j = match nums.binary_search(&(v + n as i32 - 1)) {
                Ok(j) => j + 1,
                Err(j) => j,
            };

            n - j + i
        }).min().unwrap() as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 2, 5, 3];
        let ans = 0;
        assert_eq!(Solution::min_operations(nums), ans);

        let nums = vec![1, 2, 3, 5, 6];
        let ans = 1;
        assert_eq!(Solution::min_operations(nums), ans);

        let nums = vec![1, 10, 100, 1000];
        let ans = 3;
        assert_eq!(Solution::min_operations(nums), ans);
    }
}
