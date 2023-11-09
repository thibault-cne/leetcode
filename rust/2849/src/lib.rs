pub struct Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        // edge case
        if t == 1 && sx == fx && sy == fy {
            return false;
        }

        std::cmp::max((fx - sx).abs(), (fy - sy).abs()) <= t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($sx:expr, $sy:expr, $fx:expr, $fy:expr, $t:expr, $e:expr)),*) => {
            $(
                assert_eq!(
                    Solution::is_reachable_at_time($sx, $sy, $fx, $fy, $t),
                    $e
                );
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!(
            (2, 4, 7, 7, 6, true),
            (3, 1, 7, 3, 3, false)
        );
    }
}