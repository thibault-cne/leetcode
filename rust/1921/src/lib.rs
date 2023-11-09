pub struct Solution;

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut sort = (0..dist.len())
            .map(|i| (dist[i] + speed[i] - 1) / speed[i])
            .collect::<Vec<i32>>();

        sort.sort_unstable();

        for (i, v) in sort.iter().enumerate().skip(1) {
            if v - i as i32 <= 0 {
                return i as i32;
            }
        }

        dist.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(([$($dist:expr),*], [$($speed:expr),*], $e:expr)),*) => {
            $(
                assert_eq!(
                    Solution::eliminate_maximum(
                        vec![$($dist),*],
                        vec![$($speed),*]
                    ),
                    $e
                );
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!(
            ([1, 3, 4], [1, 1, 1], 3),
            ([1, 1, 2, 3], [1, 1, 1, 1], 1),
            ([3, 2, 4], [5, 3, 2], 1),
            ([4, 3, 4], [1, 1, 2], 3),
            ([4, 3, 3, 3, 4], [1, 1, 1, 1, 4], 3),
            (
                [
                    46, 33, 44, 42, 46, 36, 7, 36, 31, 47, 38, 42, 43, 48, 48, 25, 28, 44, 49, 47,
                    29, 32, 30, 6, 42, 9, 39, 48, 22, 26, 50, 34, 40, 22, 10, 45, 7, 43, 24, 18,
                    40, 44, 17, 39, 36
                ],
                [
                    1, 2, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 7, 1, 1, 3, 2, 2, 2, 1, 2, 1, 1, 1,
                    1, 1, 1, 1, 1, 6, 1, 1, 1, 8, 1, 1, 1, 3, 6, 1, 3, 1, 1
                ],
                7
            ),
            ([3, 5, 7, 4, 5], [2, 3, 6, 3, 2], 2)
        );
    }
}
