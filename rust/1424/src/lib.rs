pub struct Solution;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();

        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                ans.push((i + j, j, nums[i][j]));
            }
        }

        ans.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        ans.into_iter().map(|(_, _, v)| v).collect::<Vec<i32>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        assert_eq!(Solution::find_diagonal_order(nums), vec![1, 4, 2, 7, 5, 3, 8, 6, 9]);
    }
}
