pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut bounds = (0_usize, s.len());
        let mut size = match s.len() / 2 {
            0 if s.is_empty() => return 0,
            0 => 1,
            l => l,
        };

        let chars: Vec<char> = s.chars().collect();

        loop {
            match Self::check_for_size(&chars, size) {
                true => bounds.0 = size,
                false => bounds.1 = size,
            }

            size = match bounds.1 - bounds.0 {
                0 => break bounds.0 as i32,
                1 if Self::check_for_size(&chars, bounds.1) => break bounds.1 as i32,
                1 => break bounds.0 as i32,
                s => s / 2 + bounds.0,
            }
        }
    }

    fn check_for_size(chars: &[char], size: usize) -> bool {
        let find = chars.windows(size).find(|s| {
            let s: String = s.iter().collect();
            Self::check_unicity(&s)
        });

        find.is_some()
    }

    fn check_unicity(s: &str) -> bool {
        !s.chars().enumerate().any(|(i, c1)| {
            let l: Vec<(usize, char)> = s
                .chars()
                .enumerate()
                .filter(|(j, c2)| c1 == *c2 && i != *j)
                .collect();

            !l.is_empty()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicity_works() {
        assert!(!Solution::check_unicity("abcabcbb"));
        assert!(Solution::check_unicity("abc"));
        assert!(Solution::check_unicity("abcdefg"));
        assert!(!Solution::check_unicity("abcdefgha"));
    }

    #[test]
    fn solution_works() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_owned()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_owned()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring("".to_owned()), 0);
        assert_eq!(Solution::length_of_longest_substring(" ".to_owned()), 1);
        assert_eq!(Solution::length_of_longest_substring("au".to_owned()), 2);
    }
}
