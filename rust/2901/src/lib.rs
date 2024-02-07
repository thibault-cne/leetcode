pub struct Solution;

impl Solution {
    pub fn get_words_in_longest_subsequence(
        n: i32,
        words: Vec<String>,
        groups: Vec<i32>,
    ) -> Vec<String> {
        let mut hamming_distances = std::collections::HashMap::new();
        let mut dp = vec![1; n as usize];
        let mut prev = vec![0; n as usize];

        (1..n as usize).for_each(|i| {
            if let Some(j) = (0..i)
                .filter(|&j| groups[i] != groups[j])
                .filter(|&j| {
                    *hamming_distances
                        .entry((i, j))
                        .or_insert(check(&words[i], &words[j]))
                })
                .filter(|&j| dp[i] <= dp[j])
                .max_by(|&x, &y| dp[x].cmp(&dp[y]))
            {
                dp[i] = dp[j] + 1;
                prev[i] = j;
            }
        });

        let (mut idx, _) = dp.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap();
        let mut ans = Vec::new();

        let mut count = dp[idx];
        while count > 0 {
            ans.push(words[idx].clone());
            idx = prev[idx];
            count -= 1;
        }

        ans.reverse();
        ans
    }
}

fn check(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    s.chars().zip(t.chars()).filter(|(a, b)| a != b).count() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tests {
        ($($n:expr, [$($words:literal),*], [$($groups:expr),*] => [$($expected:expr),*])*) => {
            $(
                {
                    let res = Solution::get_words_in_longest_subsequence(
                        $n,
                        vec![$($words.to_string()),*],
                        vec![$($groups),*]
                    );
                    assert_eq!(res, vec![$($expected.to_string()),*]);
                }
            )*
        };
    }

    #[test]
    fn tests() {
        tests! {
            3, ["bab","dab","cab"], [1, 2, 2] => ["bab","cab"]
            4, ["a","b","c","d"], [1, 2, 3, 4] => ["a","b","c","d"]
            3, ["bdb","aaa","ada"], [3, 2, 1] => ["aaa","ada"]
            5, ["ca","cb","bcd","bb","ddc"], [2,4,2,5,1] => ["ca","cb","bb"]
            13, ["cb","dc","ab","aa","ac","bb","ca","bcc","cdd","aad","bba","bc","ddb"], [12,6,10,11,4,8,9,11,2,11,3,2,5] => ["cb","ab","aa","ac","bc"]
        }
    }
}
