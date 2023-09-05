use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let vec = Self::from(n);

        let prod: i32 = vec.iter().product();
        let sum: i32 = vec.iter().sum();

        prod - sum
    }

    pub fn from(value: i32) -> Vec<i32> {
        let mut value = value;
        let mut res: VecDeque<i32> = VecDeque::new();

        loop {
            match value % 10 {
                0 if value < 10 => break,
                v => {
                    res.push_front(v);
                    value /= 10
                }
            }
        }

        if res.len() == 0 {
            res.push_front(0);
        }

        res.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(Solution::from(10), vec![1, 0]);
    }

    #[test]
    fn test() {
        assert_eq!(Solution::subtract_product_and_sum(234), 15);
        assert_eq!(Solution::subtract_product_and_sum(4421), 21);
    }
}
