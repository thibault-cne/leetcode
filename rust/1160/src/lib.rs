pub struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut ans = 0;

        for c in chars.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for word in words {
            let mut word_map = std::collections::HashMap::new();

            for c in word.chars() {
                *word_map.entry(c).or_insert(0) += 1;
            }

            if word_map.len() > map.len() {
                continue;
            }

            if !word_map
                .iter()
                .any(|(k, v)| !map.contains_key(k) || map.get(k).unwrap() < v)
            {
                ans += word.len() as i32;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::count_characters(
                vec![
                    "cat".to_string(),
                    "bt".to_string(),
                    "hat".to_string(),
                    "tree".to_string()
                ],
                "atach".to_string()
            ),
            6
        );
        assert_eq!(
            Solution::count_characters(
                vec![
                    "hello".to_string(),
                    "world".to_string(),
                    "leetcode".to_string()
                ],
                "welldonehoneyr".to_string()
            ),
            10
        );
    }
}
