use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_the_difference_xor(s: String, t: String) -> char {
        let mut result = 0;

        s.bytes().chain(t.bytes()).for_each(|b| result ^= b);

        result as char
    }

    pub fn find_the_difference(s: String, t: String) -> char {
        let mut map_s: HashMap<char, usize> = HashMap::new();
        let mut map_t: HashMap<char, usize> = HashMap::new();

        s.chars().for_each(|cs| {
            *map_s.entry(cs).or_insert(0) += 1;
        });

        t.chars().for_each(|ct| {
            *map_t.entry(ct).or_insert(0) += 1;
        });

        for (key, value) in map_t.iter() {
            if let Some(val) = map_s.get(key) {
                if val != value {
                    return *key;
                }
            } else {
                return *key;
            }
        }

        'a'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
            'e'
        );
    }
}
