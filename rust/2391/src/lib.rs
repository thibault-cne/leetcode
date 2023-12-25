pub struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut flags: i32 = 0;

        for (t, house) in travel
            .into_iter()
            .rev()
            .chain(std::iter::once(0))
            .zip(garbage.into_iter().rev())
        {
            for b in house.bytes() {
                flags |= 1 << (b % 5);

                if flags == 8 {
                    break;
                }
            }

            ans += house.len() as i32 + flags.count_ones() as i32 * t;
        }

        ans
    }

    pub fn garbage_collection_first(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last_metal = 0;
        let mut last_glass = 0;
        let mut last_paper = 0;

        for (i, v) in garbage.iter().enumerate() {
            ans += match v.chars().filter(|&c| c == 'M').count() {
                0 => 0,
                cpt => {
                    last_metal = i;
                    cpt as i32
                }
            };
            ans += match v.chars().filter(|&c| c == 'G').count() {
                0 => 0,
                cpt => {
                    last_glass = i;
                    cpt as i32
                }
            };
            ans += match v.chars().filter(|&c| c == 'P').count() {
                0 => 0,
                cpt => {
                    last_paper = i;
                    cpt as i32
                }
            };
        }

        ans += travel[0..last_glass].iter().sum::<i32>();
        ans += travel[0..last_metal].iter().sum::<i32>();
        ans += travel[0..last_paper].iter().sum::<i32>();

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($(([$($garbage:literal),*], [$($time:expr),*], $expect:expr)),*) => {
            $(
                assert_eq!(
                    Solution::garbage_collection(
                        vec![$($garbage.to_string()),*],
                        vec![$($time),*]
                    ),
                    $expect
                );
            )*
        };
    }

    #[test]
    fn test() {
        test!(
            (["G", "P", "GP", "GG"], [2, 4, 3], 21),
            (["MMM", "PGM", "GP"], [3, 10], 37)
        );
    }
}
