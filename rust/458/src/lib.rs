pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let attempts = (minutes_to_test / minutes_to_die) as f64 + 1_f64;

        let res = (buckets as f64).log2() / attempts.log2();

        res.ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($b:expr, $mtd:expr, $mtt:expr, $e:expr)),*) => {
            $(
                assert_eq!(Solution::poor_pigs($b, $mtd, $mtt), $e);
)*
        };
    }

    #[test]
    fn test() {
        test_eq!((4, 15, 15, 2), (4, 15, 30, 2));
    }
}
