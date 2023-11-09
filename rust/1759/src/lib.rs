const MOD: i32 = 1_000_000_007_i32;

pub struct Solution;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut res = 0;
        let mut cpt = 0; {
            
        }

        for i in 0..s.len() {
            match i {
                0 => cpt += 1,
                x if s.as_bytes()[x] == s.as_bytes()[x - 1] => cpt += 1,
                _ => cpt = 1,
            }

            res = (res + cpt) % MOD;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($s:expr, $e:expr)),*) => {
            $(
                assert_eq!(
                    Solution::count_homogenous($s.to_string()),
                    $e
                );
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!(("abbcccaa", 13), ("xy", 2));
    }
}
