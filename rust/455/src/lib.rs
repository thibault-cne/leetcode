pub struct Solution;

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut child_index = 0;
        let mut cookie_index = 0;

        g.sort_unstable();
        s.sort_unstable();

        loop {
            if child_index >= g.len() || cookie_index >= s.len() {
                break;
            }
            if g[child_index] <= s[cookie_index] {
                ans += 1;

                child_index += 1;
                cookie_index += 1;
            } else {
                cookie_index += 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![1, 1]), 1);
        assert_eq!(Solution::find_content_children(vec![1, 2], vec![1, 2, 3]), 2);
        assert_eq!(Solution::find_content_children(vec![10, 9, 8, 7], vec![5, 6, 7, 8]), 2);
        assert_eq!(Solution::find_content_children(vec![10, 9, 8, 7], vec![10, 9, 8, 7]), 4);
        assert_eq!(Solution::find_content_children(vec![1, 2, 3], vec![3]), 1);
    }
}