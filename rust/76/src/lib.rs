pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let sb = s.as_bytes();
        let mut t_len = t.len();
        let mut left = 0;
        let mut right = 0;
        let mut counts = [0; (b'z' - b'A') as usize + 1];
        let mut win = None;

        macro_rules! counts {
            [$i:expr] => {
                counts[($i - b'A') as usize]
            };
        }

        t.bytes().for_each(|b| counts![b] += 1);

        while right < s.len() {
            if counts![sb[right]] > 0 {
                t_len -= 1;
            }
            counts![sb[right]] -= 1;
            right += 1;
            while t_len == 0 {
                if right - left < win.map_or(std::usize::MAX, |(left, right)| right - left) {
                    win = Some((left, right));
                }
                counts![sb[left]] += 1;
                if counts![sb[left]] > 0 {
                    t_len += 1;
                }
                left += 1;
            }
        }

        win.map_or("".into(), |(left, right)| s[left..right].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tests {
        ($($s:literal, $t:literal => $res:literal),*) => {
            $(
                assert_eq!(Solution::min_window($s.to_string(), $t.to_string()), $res.to_string());
            )*
        };
    }

    #[test]
    fn test() {
        tests!(
            "ADOBECODEBANC", "ABC" => "BANC",
            "a", "a" => "a",
            "a", "aa" => ""
        );
    }
}
