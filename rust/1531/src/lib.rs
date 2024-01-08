pub struct Solution;

impl Solution {
    pub fn get_length_of_optimal_compression(s: impl AsRef<[u8]>, k: i32) -> i32 {
        assert!(k >= 0);
        let k = k as usize;
        let s = s.as_ref();

        if s.is_empty() || s.len() == k {
            return 0;
        }

        let mut cache = std::collections::HashMap::new();
        solve(s, &mut cache, (0, 0), 0, k)
    }
}

macro_rules! len {
    ($count:expr) => {
        match $count {
            1 => 1,
            2..=9 => 2,
            10..=99 => 3,
            100..=999 => 4,
            1000..=9999 => 5,
            _ => panic!("invalid input: {}", $count),
        }
    };
}

fn solve(
    s: &[u8],
    cache: &mut std::collections::HashMap<(usize, usize, (u8, i32)), i32>,
    state: (u8, i32),
    idx: usize,
    k: usize,
) -> i32 {
    if idx == s.len() {
        return 0;
    }

    if let Some(count) = cache.get(&(k, idx, state)).copied() {
        return count;
    }

    let chr = s[idx];
    let next;
    let inc;

    if chr == state.0 {
        next = (state.0, state.1 + 1);
        inc = len!(next.1) - len!(state.1);
    } else {
        next = (chr, 1);
        inc = 1;
    }

    let mut length = inc + solve(s, cache, next, idx + 1, k);

    if k > 0 && inc > 0 {
        length = length.min(solve(s, cache, state, idx + 1, k - 1));
    }

    cache.insert((k, idx, state), length);

    length
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($(($s:literal, $k:expr, $expect:expr)),*) => {
            $(
                assert_eq!(Solution::get_length_of_optimal_compression($s.to_string(), $k), $expect);
            )*
        };
    }

    #[test]
    fn test() {
        test!(("aaabcccd", 2, 4), ("aabbaa", 2, 2), ("aaaaaaaaaaa", 0, 3));
    }
}
