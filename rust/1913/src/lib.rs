pub struct Solution;

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let (a, b, c, d) = nums
            .iter()
            .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN), |mut acc, &v| {
                if v > acc.3 {
                    acc.2 = acc.3;
                    acc.3 = v;
                } else if v > acc.2 {
                    acc.2 = v;
                }

                if v < acc.0 {
                    acc.1 = acc.0;
                    acc.0 = v;
                } else if v < acc.1 {
                    acc.1 = v;
                }

                acc
            });

        (d * c) - (b * a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_product_difference(vec![5, 6, 2, 7, 4]), 34);
        assert_eq!(
            Solution::max_product_difference(vec![1, 6, 7, 5, 2, 4, 10, 6, 4]),
            68
        );
        assert_eq!(
            Solution::max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]),
            64
        );
        assert_eq!(
            Solution::max_product_difference(vec![1, 1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
