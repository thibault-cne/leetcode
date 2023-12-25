pub struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let size = s.len() - 1;
        let mut dp = vec![0; size];
        let mut count = (0, 0);

        (0..size)
            .zip(s.chars().take(size).zip(s.chars().rev().take(size)))
            .for_each(|(i, (c1, c2))| {
                if c1 == '0' {
                    count.0 += 1;
                }
                if c2 == '1' {
                    count.1 += 1;
                }

                dp[i] += count.0;
                dp[size - (i + 1)] += count.1;
            });

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s = "011101".to_string();
        let expected = 5;
        assert_eq!(Solution::max_score(s), expected);
    }

    #[test]
    fn test_2() {
        let s = "00111".to_string();
        let expected = 5;
        assert_eq!(Solution::max_score(s), expected);
    }

    #[test]
    fn test_3() {
        let s = "1111".to_string();
        let expected = 3;
        assert_eq!(Solution::max_score(s), expected);
    }

    #[test]
    fn test_4() {
        let s = "00".to_string();
        let expected = 1;
        assert_eq!(Solution::max_score(s), expected);
    }

    #[test]
    fn test_5() {
        let s = "0100".to_string();
        let expected = 2;
        assert_eq!(Solution::max_score(s), expected);
    }

    #[test]
    fn test_6() {
        let s = "111000".to_string();
        let expected = 2;
        assert_eq!(Solution::max_score(s), expected);
    }
}
