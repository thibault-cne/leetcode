use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut map: Vec<usize> = vec![0; 26];
        let mut cnt = 0;
        let mut hash: HashSet<usize> = HashSet::new();

        s.as_bytes().iter().for_each(|&c| {
            map[c as usize - 97] += 1;
        });

        map.sort_by(|a, b| b.cmp(a));
        let map: Vec<usize> = map.into_iter().filter(|i| *i != 0_usize).collect();

        for f in map {
            let mut f = f;

            if !hash.contains(&f) {
                hash.insert(f);
                continue;
            }

            loop {
                if f == 0 || !hash.contains(&f) {
                    hash.insert(f);
                    break;
                }

                f -= 1;
                cnt += 1;
            }
        }

        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let tests: Vec<(&str, i32)> = vec![("aab", 0), ("aaabbbcc", 2), ("ceabaacb", 2)];

        tests.iter().for_each(|(input, expect)| {
            assert_eq!(Solution::min_deletions(input.to_string()), *expect);
        })
    }
}
