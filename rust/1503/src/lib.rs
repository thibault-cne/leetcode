pub struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let mut max = 0;

        for f in left {
            if f > max {
                max = f;
            }
        }

        for f in right {
            if n - f > max {
                max = n - f;
            }
        }

        max
    }

    pub fn get_last_moment_sort(n: i32, mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
        left.sort();
        right.sort();

        match (left.len(), right.len()) {
            (0, 0) => 0,
            (0, _) => n - right[0],
            (_, 0) => left[left.len() - 1],
            _ => std::cmp::max(n - right[0], left[left.len() - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($n:expr, [$($d1:expr),*], [$($d2:expr),*], $e:expr)),*) => {
            $(
                assert_eq!(Solution::get_last_moment($n, vec![$($d1),*], vec![$($d2),*]), $e);
            )*
        }
    }

    #[test]
    fn test() {
        test_eq!(
            (4, [4, 3], [0, 1], 4),
            (7, [0,1,2,3,4,5,6,7], [], 7),
            (7, [], [0,1,2,3,4,5,6,7], 7)
        );
    }
}