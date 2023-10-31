pub struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut res = vec![pref[0]; pref.len()];

        for i in 1..pref.len() {
            res[i] = pref[i - 1] ^ pref[i];
        }

        res
    }

    pub fn find_array_scan(pref: Vec<i32>) -> Vec<i32> {
        pref.iter()
            .scan(0, |prev, &x| {
                let res = *prev ^ x;
                *prev = x;
                Some(res)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(([$($d1:expr),*], [$($d2:expr),*])),*) => {
            $(
                assert_eq!(Solution::find_array(vec![$($d1),*]), vec![$($d2),*]);
            )*
        };
    }

    #[test]
    fn func() {
        test_eq!(([5, 2, 0, 3, 1], [5, 7, 2, 3, 2]), ([13], [13]));
    }
}
