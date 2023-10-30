pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // if x is negative than it's not a palindrome
        if x < 0 {
            return false;
        }

        let mut acc = Vec::new();
        let mut n = x;

        loop {
            if n == 0 {
                break;
            }

            acc.push(n % 10);
            n /= 10;
        }

        let len = acc.len();
        let res: i32 = acc
            .iter()
            .enumerate()
            .map(|(i, v)| v * 10_i32.pow((len - i - 1) as u32))
            .sum();

        res == x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        assert!(Solution::is_palindrome(121));
        assert!(Solution::is_palindrome(11));
        assert!(Solution::is_palindrome(1));
        assert!(Solution::is_palindrome(1234321));
    }

    #[test]
    fn invalid() {
        assert!(!Solution::is_palindrome(122));
        assert!(!Solution::is_palindrome(-11));
        assert!(!Solution::is_palindrome(10));
        assert!(!Solution::is_palindrome(1234322));
    }
}
