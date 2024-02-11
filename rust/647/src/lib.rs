pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        (0..s.len() * 2 - 1)
            .map(|i| {
                (0..=i / 2)
                    .rev()
                    .zip(((i + 1) / 2)..s.len())
                    .take_while(|(j, k)| s[*j] == s[*k])
                    .count() as i32
            })
            .sum()
    }
}
