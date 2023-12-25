pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        // println!("{s}");
        if s.starts_with('0') {
            return 0;
        }

        let mut res = 1;
        let mut single_digit_last = 1;
        let mut last_digit = s.as_bytes()[0] as char;

        for c in s.chars().skip(1) {
            /* println!(
                "res: {}, current_digit: {}, last_digit: {}, single_digit_last: {}",
                res, c, last_digit, single_digit_last
            ); */
            if c == '0' {
                if is_valid_number(last_digit, c) {
                    res = single_digit_last;
                    single_digit_last = 0;
                } else {
                    return 0;
                }
            } else if is_valid_number(last_digit, c) {
                let temp = res;
                res += single_digit_last;
                single_digit_last = temp;
            } else {
                single_digit_last = res;
            }

            last_digit = c;
        }

        res
    }
}

fn is_valid_number(c1: char, c2: char) -> bool {
    match (c1, c2) {
        ('1', _) => true,
        ('2', n) if n < '7' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_decodings("12".into()), 2);
        assert_eq!(Solution::num_decodings("226".into()), 3);
        assert_eq!(Solution::num_decodings("06".into()), 0);
        assert_eq!(Solution::num_decodings("60".into()), 0);
        assert_eq!(Solution::num_decodings("220".into()), 1);
        assert_eq!(Solution::num_decodings("2261".into()), 3);
        assert_eq!(Solution::num_decodings("22610".into()), 3);
        assert_eq!(Solution::num_decodings("22611".into()), 6);
    }

    #[test]
    fn valid_number() {
        assert!(is_valid_number('2', '6'));
        assert!(is_valid_number('2', '1'));
        assert!(is_valid_number('2', '0'));
        assert!(is_valid_number('1', '6'));
    }
}
