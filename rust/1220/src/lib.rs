pub struct Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let mut vowels: [i64; 5] = [1; 5];

        for _ in 1..n {
            let a_next = vowels[1];
            let e_next = (vowels[0] + vowels[2]) % MOD;
            let i_next = (vowels[0] + vowels[1] + vowels[3] + vowels[4]) % MOD;
            let o_next = (vowels[2] + vowels[4]) % MOD;
            let u_next = vowels[0];

            vowels[0] = a_next;
            vowels[1] = e_next;
            vowels[2] = i_next;
            vowels[3] = o_next;
            vowels[4] = u_next;
        }

        (vowels.iter().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($n:expr, $e:expr)),*) => {
            $(
                assert_eq!(Solution::count_vowel_permutation($n), $e);
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!((1, 5), (2, 10), (5, 68));
    }
}
