pub struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        arr.sort_unstable();

        arr[0] = 1;
        let mut prev = 0;

        for num in &mut arr {
            if *num - prev > 1 {
                *num = prev + 1;
            }

            prev = *num;
        }

        arr[arr.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
            3
        );
    }
}
