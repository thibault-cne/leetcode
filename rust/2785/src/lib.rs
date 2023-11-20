pub struct Solution;

impl Solution {
    pub fn sort_vowels(mut s: String) -> String {
        let mut vowels = Vec::new();
        let n = s.len();

        for c in s.chars() {
            if is_vowel(c) {
                vowels.push(c as u8);
            }
        }

        vowels.sort_unstable();

        unsafe {
            let vec = s.as_mut_vec();
            let mut index = 0;

            for i in 0..n {
                if is_vowel(vec[i] as char) {
                    vec[i] = vowels[index];
                    index += 1;
                }
            }
        }

        s
    }

    pub fn sort_vowels_first_attempt(s: String) -> String {
        let mut vowels = Vec::new();

        for c in s.chars() {
            if is_vowel(c) {
                vowels.push(c as u8);
            }
        }

        vowels.sort_unstable();

        let mut res = Vec::with_capacity(s.len());
        let mut index = 0;

        for c in s.chars() {
            if is_vowel(c) {
                res.push(vowels[index] as char);
                index += 1;
            } else {
                res.push(c);
            }
        }

        res.into_iter().collect()
    }
}

fn is_vowel(c: char) -> bool {
    const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    VOWELS.contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($s:literal, $e:literal)),*) => {
            $(
                assert_eq!(Solution::sort_vowels($s.to_string()), $e.to_string());
            )*
        };
    }

    #[test]
    fn test_1() {
        test_eq!(
            ("lEetcOde", "lEOtcede"),
            ("abAcD", "AbacD"),
            ("", ""),
            ("aA", "Aa"),
            ("OaEe", "EOae"),
            ("lYmpH", "lYmpH")
        );
    }
}
