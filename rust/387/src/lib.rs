pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();

        s.chars().enumerate().for_each(|(i, c)| {
            map.entry(c).or_insert((i, 0)).1 += 1;
        });

        map.into_values()
            .filter(|&(_, c)| c == 1)
            .min_by_key(|&(i, _)| i)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tests {
        ($($s:literal => $expected:expr),*) => {
            $(
                assert_eq!(Solution::first_uniq_char($s.to_string()), $expected);
            )*
        };
    }

    #[test]
    fn tests() {
        tests!(
            "leetcode" => 0,
            "loveleetcode" => 2,
            "aabbcc" => -1
        );
    }
}
