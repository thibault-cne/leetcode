pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut ans = 0;
        let slice = s.as_bytes();

        for b in 97_u8..=122 {
            let (i, j) = (
                slice.iter().position(|c| *c == b),
                slice.iter().rposition(|c| *c == b),
            );

            let mut set = std::collections::HashSet::new();

            if let (Some(i), Some(j)) = (i, j) {
                for i in slice.iter().take(j).skip(i + 1) {
                    set.insert(i);
                }

                ans += set.len();
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($s:literal, $e:expr)),*) => {
            $(
                assert_eq!(Solution::count_palindromic_subsequence($s.to_string()), $e);
            )*
        };
    }

    #[test]
    fn test() {
        test_eq! {
            ("aabca", 3),
            ("adc", 0),
            ("bbcbaba", 4)
        }
    }
}
