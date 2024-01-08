pub struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut last_ballon = colors.as_bytes()[0];
        let mut start = 0;

        colors
            .as_bytes()
            .iter()
            .enumerate()
            .skip(1)
            .for_each(|(i, &b)| {
                if b != last_ballon {
                    if i - start > 1 {
                        let max = (start..i).map(|i| needed_time[i]).max().unwrap();
                        ans += (start..i).map(|i| needed_time[i]).sum::<i32>() - max;
                    }

                    start = i;
                    last_ballon = b;
                }
            });

        // Handle last loop
        if (colors.len() - start) > 1 {
            let max = (start..colors.len()).map(|i| needed_time[i]).max().unwrap();
            ans += (start..colors.len()).map(|i| needed_time[i]).sum::<i32>() - max;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let colors = "abaac".into();
        let needed_time = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::min_cost(colors, needed_time), 3);

        let colors = "abc".into();
        let needed_time = vec![1, 2, 3];
        assert_eq!(Solution::min_cost(colors, needed_time), 0);

        let colors = "aabaa".into();
        let needed_time = vec![1, 2, 3, 4, 1];
        assert_eq!(Solution::min_cost(colors, needed_time), 2);
    }
}
