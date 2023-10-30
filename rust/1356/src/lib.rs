pub struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()).then(a.cmp(b)));

        arr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ([$($d1:expr),*], [$($d2:expr),*]) => {
            assert_eq!(Solution::sort_by_bits(vec![$($d1),*]), vec![$($d2),*]);
        };
    }

    #[test]
    fn test() {
        test_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8], [0, 1, 2, 4, 8, 3, 5, 6, 7]);
        test_eq!(
            [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1],
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
