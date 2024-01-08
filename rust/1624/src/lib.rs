pub struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;

        (0..s.len()).for_each(|i| {
            if let Some(end) = s.chars().rev().position(|c| c == s.as_bytes()[i] as char) {
                max = std::cmp::max(max, (s.len() - end - 1) as i32 - i as i32 - 1);
            }
        });

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($(($s:literal, $expect:expr)),*) => {
            $(
                assert_eq!(Solution::max_length_between_equal_characters($s.into()), $expect);
            )*
        };
    }

    #[test]
    fn test() {
        test!(
            ("aa", 0),
            ("abca", 2),
            ("cbzxy", -1),
            ("hohoho", 3),
            ("merrychristmasandahappynewyear", 26),
            ("a", -1),
            ("abcdefghijklmnopqrstuvxyza", 24),
            ("aaaaaaaaaaaaaaabbbbbbbbbbbbbbbbccccccccccccccccccfddddddddddddddddeeeeeeeeeeeeeeeeeeffffffffffffffffggggggggggggggggggggghhhhhhhhhhhhhhhhhhhfiiiiiiiiiiiiiiijjjjjjjjjjjjjjjjjjjkkkkkkkkkkkkkkkkkkkklllllllllllllllllllllllmmmmmmmmmmmmmmmmmmmm", 90)
        );
    }
}
