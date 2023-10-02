pub struct Solution;

impl Solution {
    pub fn two_pointers(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut index = 0;

        for i in 0..chars.len() {
            if i == chars.len() - 1 || chars[i] == ' ' {
                let mut end = if i == chars.len() - 1 && chars[i] != ' ' { i + 1 } else { i };
                while index < end {
                    chars.swap(index, end - 1);
                    index += 1;
                    end -= 1;
                }
                index = i + 1;
            }
        }

        chars.iter().collect()
    }

    pub fn reverse_words(s: String) -> String {
        let mut reverse = s.clone();
        let mut last = 0;

        for (i, c) in s.chars().enumerate() {
            if c == ' ' {
                reverse.replace_range(last..i, &s[last..i].chars().rev().collect::<String>());
                last = i + 1;
            } else if i == s.len() - 1 {
                reverse.replace_range(last..i + 1, &s[last..i + 1].chars().rev().collect::<String>());
            }
        }

        reverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }

    #[test]
    fn test_two_pointers() {
        assert_eq!(
            Solution::two_pointers("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc".to_string()
        );
    }
}