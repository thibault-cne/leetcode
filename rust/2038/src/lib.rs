pub struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut a = 0;
        let mut b = 0;

        for i in 1..colors.len()-1 {
            if &colors[i-1..=i+1] == "AAA" {
                a += 1;
            }
            else if &colors[i-1..=i+1] == "BBB" {
                b += 1;
            }
        }

        a > b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::winner_of_game("AAABABB".into()));
        assert!(Solution::winner_of_game("AAAAABBB".into()));
        assert!(!Solution::winner_of_game("AA".into()));
        assert!(!Solution::winner_of_game("ABBBBBBBAAA".into()));
    }
}
