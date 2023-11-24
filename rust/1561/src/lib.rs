pub struct Solution;

impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        let n = piles.len() / 3;
        piles.sort_unstable();
        piles.iter().skip(n).step_by(2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
        assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
        assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
    }
}