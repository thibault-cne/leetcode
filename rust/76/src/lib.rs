pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".to_string();
        }

        let mut left = 0;
        let mut right = 0;
        let mut t_map = std::collections::HashMap::new();

        t.bytes().for_each(|c| {
            *t_map.entry(c).or_insert(0) += 1;
        });

        let s = s.as_bytes();
        let mut s_map = std::collections::HashMap::new();
        let mut min = (0, s.len() as i32 + 1);

        while left < s.len() as i32 && right < s.len() as i32 {
            while right < s.len() as i32 && !map_contains(&s_map, &t_map) {
                *s_map.entry(s[right as usize]).or_insert(0) += 1;
                right += 1;
            }

            while left < s.len() as i32 && map_contains(&s_map, &t_map) {
                *s_map.entry(s[left as usize]).or_insert(0) -= 1;
                left += 1;
            }

            if right - (left - 1) < min.1 - min.0 {
                min = (left - 1, right);
            }
        }

        if min.1 - min.0 == s.len() as i32 + 1 {
            "".to_string()
        } else {
            let min = (min.0 as usize, min.1 as usize);
            s[min.0..min.1].iter().map(|&c| c as char).collect()
        }
    }
}

fn map_contains(
    left: &std::collections::HashMap<u8, i32>,
    right: &std::collections::HashMap<u8, i32>,
) -> bool {
    right
        .iter()
        .all(|(k, v)| left.get(k).map_or(false, |x| x >= v))
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
