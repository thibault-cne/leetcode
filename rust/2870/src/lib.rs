pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut calc = |cpt| {
            ans += cpt / 3;
            if cpt % 3 != 0 {
                ans += 1;
            }
        };
        nums.sort_unstable();

        match nums.iter().try_fold((0, 0), |acc, &v| {
            if v != acc.0 && acc.1 != 0 {
                if acc.1 == 1 {
                    return None;
                }
                calc(acc.1);
                Some((v, 1))
            } else {
                Some((v, acc.1 + 1))
            }
        }) {
            None => -1,
            Some((_, cpt)) => {
                if cpt == 1 {
                    -1
                } else {
                    calc(cpt);
                    ans
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_operations(vec![2, 3, 3, 2, 2, 4, 2, 3, 4]), 4);
    }
}
