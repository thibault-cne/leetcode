pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut count = [0; 75];

        macro_rules! count {
           [$i:expr] => {
                count[$i as usize - '0' as usize]
            };
        }

        s.chars().for_each(|c| count![c] += 1);

        let mut count = count
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c > 0)
            .collect::<Vec<_>>();

        count.sort_unstable_by(|x, y| y.1.cmp(x.1));

        count
            .iter()
            .map(|&(c, &n)| vec![(c as u8 + b'0') as char; n].iter().collect::<String>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "tree".to_string();
        assert_eq!(Solution::frequency_sort(s), "eert".to_string());
    }
}
