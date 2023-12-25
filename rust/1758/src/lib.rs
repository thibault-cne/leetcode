pub struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let curr_byte = |prev| if prev == 1 { '1' } else { '0' };
        let (mut res0, mut res1) = (0, 0);
        let mut curr = 0;

        s.chars().for_each(|c| {
            if c == curr_byte(curr) {
                res1 += 1;
            } else {
                res0 += 1;
            }

            curr = 1 - curr;
        });

        res0.min(res1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_operations("0100".into()), 1);
        assert_eq!(Solution::min_operations("01101010".into()), 2);
        assert_eq!(Solution::min_operations("0101".into()), 0);
        assert_eq!(Solution::min_operations("010".into()), 0);
        assert_eq!(Solution::min_operations("010011".into()), 2);
        assert_eq!(Solution::min_operations("0100110".into()), 2);
        assert_eq!(Solution::min_operations("01000110".into()), 3);
        assert_eq!(Solution::min_operations("01".into()), 0);
        assert_eq!(Solution::min_operations("1111".into()), 2);
    }
}
