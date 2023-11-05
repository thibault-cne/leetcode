use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn get_winner(mut arr: Vec<i32>, k: i32) -> i32 {
        if k as usize >= arr.len() {
            arr.sort();
            return arr[arr.len() - 1];
        }

        let mut win = 0;
        let mut winner = arr[0];
        let mut queue = VecDeque::from(arr);

        // Pop first element
        queue.pop_front();

        loop {
            if win == k {
                break;
            }

            if let Some(k) = queue.pop_front() {
                if winner > k {
                    queue.push_back(k);
                    win += 1;
                } else {
                    queue.push_back(winner);
                    winner = k;
                    win = 1;
                }
            }
        }

        winner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(([$($arr:expr),*], $k:expr, $e:expr)),*) => {
            $(
                assert_eq!(Solution::get_winner(vec![$($arr),*], $k), $e);
            )*
        }
    }

    #[test]
    fn test() {
        test_eq!(
            ([2, 1, 3, 5, 4, 6, 7], 2, 5),
            ([3, 2, 1], 10, 3),
            ([1, 9, 8, 2, 3, 7, 6, 4, 5], 7, 9),
            ([1, 11, 22, 33, 44, 55, 66, 77, 88, 99], 100000, 99)
        );
    }
}
