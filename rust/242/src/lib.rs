pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s = Vec::from(s);
        let mut t = Vec::from(t);

        s.sort_unstable();
        t.sort_unstable();

        !s.iter().zip(t.iter()).any(|(l, r)| l != r)
    }
}
