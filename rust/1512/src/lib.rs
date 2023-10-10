pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; 101];

        nums.iter().for_each(|&i| count[i as usize] += 1);

        count.iter().for_each(|&i| ans += i * (i - 1) / 2);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
