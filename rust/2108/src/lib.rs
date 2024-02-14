pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .iter()
            .find(|word| word.as_str() == word.chars().rev().collect::<String>().as_str())
            .cloned()
            .unwrap_or_default()
    }
}
