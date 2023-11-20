pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut max = -1;

        for i in 0..nums.len() / 2 {
            max = std::cmp::max(max, nums[i] + nums[nums.len() - (i + 1)]);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($(([$($nums:expr),*], $expect:expr)),*) => {
            $(
                assert_eq!(Solution::min_pair_sum(vec![$($nums),*]), $expect);
            )*
        };
    }

    #[test]
    fn test() {
        test!(([3, 5, 2, 3], 7), ([3, 5, 4, 2, 4, 6], 8));
    }
}
