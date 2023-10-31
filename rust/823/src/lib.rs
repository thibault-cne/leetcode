use std::collections::HashMap;

pub struct Solution;

const MOD: i64 = 1e9 as i64 + 7;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let mut dp: HashMap<i32, i64> = HashMap::new();

        for i in 0..arr.len() {
            dp.insert(arr[i], 1);

            for j in 0..i {
                if arr[i] % arr[j] == 0 && dp.contains_key(&(arr[i] / arr[j])) {
                    let left = *dp.get(&arr[j]).unwrap() as i64;
                    let right = *dp.get(&(arr[i] / arr[j])).unwrap() as i64;
                    dp.entry(arr[i]).and_modify(|v| {
                       *v = (*v + left * right) % MOD;
                    });
                }
            }
        }

        (dp.into_values().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(([$($d1:expr),*], $e:expr)),*) => {
            $(
                assert_eq!(Solution::num_factored_binary_trees(vec![$($d1),*]), $e);
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!(
            ([2], 1),
            ([2,4], 3),
            ([2,4,5,10], 7)
        );
    }
}
