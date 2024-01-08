pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut ans = 0;

        bank.iter().fold(0, |acc, row| {
            match row.chars().filter(|&c| c == '1').count() {
                0 => acc,
                devices => {
                    ans += acc * devices;
                    devices
                }
            }
        });

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ([$($row:literal),*], $expected:expr) => {
            assert_eq!(
                Solution::number_of_beams(vec![$($row.to_string()),*]),
                $expected
            );
        }
    }

    #[test]
    fn test() {
        test!(
            [
                "011001",
                "000000",
                "010100",
                "001000"
            ],
            8
        );
        test!(
            [
                "000",
                "111",
                "000"
            ],
            0
        );
    }
}
