pub struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut table = std::collections::HashMap::new();

        words
            .iter()
            .for_each(|w| w.chars().for_each(|c| *table.entry(c).or_insert(0) += 1));

        for v in table.into_values() {
            if v % words.len() != 0 {
                return false;
            }
        }

        true
    }
}
