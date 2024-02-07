pub struct Solution;

impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        strs.sort_unstable_by(|a, b| anagram(a, b));

        strs.into_iter()
            .fold(
                (vec![vec![]], None),
                |(mut acc, last): (Vec<Vec<String>>, Option<String>), s| {
                    if let Some(last) = last {
                        match anagram(&last, &s) {
                            std::cmp::Ordering::Equal => {
                                acc.last_mut().unwrap().push(s.clone());
                                (acc, Some(s))
                            }
                            _ => {
                                acc.push(vec![s.clone()]);
                                (acc, Some(s))
                            }
                        }
                    } else {
                        acc.last_mut().unwrap().push(s.clone());
                        (acc, Some(s))
                    }
                },
            )
            .0
    }
}

fn anagram(s: &str, t: &str) -> std::cmp::Ordering {
    if s.len() != t.len() {
        return s.len().cmp(&t.len());
    }

    let mut counts = [0; 26];

    s.chars().zip(t.chars()).for_each(|(sc, tc)| {
        counts[(sc as u8 - b'a') as usize] += 1;
        counts[(tc as u8 - b'a') as usize] -= 1;
    });

    counts
        .iter()
        .find(|&&c| c != 0)
        .map_or(std::cmp::Ordering::Equal, |&c| c.cmp(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! string_vec {
        [$($strs:literal),*] => {vec![$($strs.to_string()),*]};
    }

    macro_rules! tests {
        ($([$($strs:literal),*] => $expected:expr);*;) => {
            $(
                {
                    let mut res = Solution::group_anagrams(vec![$($strs.to_string()),*]);
                    res.iter_mut().for_each(|v| v.sort_unstable());
                    res.sort_unstable();

                    let mut expected = $expected;
                    expected.iter_mut().for_each(|v| v.sort_unstable());
                    expected.sort_unstable();

                    assert_eq!(res, expected);
                }
            )*
        };
    }

    #[test]
    fn tests() {
        tests!(
            ["eat", "tea", "tan", "ate", "nat", "bat"] => vec![
                string_vec!["bat"],
                string_vec!["nat", "tan"],
                string_vec!["ate", "eat", "tea"]
            ];
            [""] => vec![string_vec![""]];
            ["a"] => vec![string_vec!["a"]];
        );
    }

    #[test]
    fn anagram_cmp() {
        use std::cmp::Ordering::*;

        assert_eq!(anagram("a", ""), Greater);
        assert_eq!(anagram("", "a"), Less);
        assert_eq!(anagram("a", "a"), Equal);
        assert_eq!(anagram("a", "b"), Greater);
        assert_eq!(anagram("b", "a"), Less);
    }
}
