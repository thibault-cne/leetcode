pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let mut max_str = &s[..1];

        for i in 0..s.len() {
            let odd = expand_from_center(&s, i as isize, i as isize);
            let even = expand_from_center(&s, i as isize, i as isize + 1);

            if odd.len() > max_str.len() {
                max_str = odd;
            }

            if even.len() > max_str.len() {
                max_str = even;
            }
        }

        max_str.to_string()
    }
}

fn expand_from_center<'a>(s: &'a str, mut left: isize, mut right: isize) -> &'a str {
    let bytes = s.as_bytes();

    loop {
        if left < 0 || right >= s.len() as isize || bytes[left as usize] != bytes[right as usize] {
            break;
        }

        left -= 1;
        right += 1;
    }

    &s[(left + 1) as usize..right as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_eq {
        ($(($v:literal, $e:literal)),*) => {
            $(
                assert_eq!(Solution::longest_palindrome($v.to_string()), $e.to_string());
            )*
        };
    }

    #[test]
    fn test() {
        test_eq!(
            ("babad", "bab"),
            ("cbbd", "bb"),
            ("a", "a")
        );
    }
}