pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &x| acc ^ x) ^ (0..=nums.len() as i32).fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
