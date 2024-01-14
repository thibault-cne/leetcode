pub struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let filter = |c| matches!(c, 'a' | 'A' | 'e' | 'E' | 'i' | 'I' | 'o' | 'O' | 'u' | 'U');

        s.chars().take(s.len() / 2).filter(|c| filter(*c)).count()
            == s.chars().skip(s.len() / 2).filter(|c| filter(*c)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::halves_are_alike("book".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::halves_are_alike("textbook".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::halves_are_alike("MerryChristmas".to_string()));
    }

    #[test]
    fn case4() {
        assert!(Solution::halves_are_alike("AbCdEfGh".to_string()));
    }
}
