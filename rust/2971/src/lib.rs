pub struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let mut prefix_sum: i64 = nums.iter().take(nums.len() - 1).map(|&e| e as i64).sum();

        for i in (2..nums.len()).rev() {
            if prefix_sum > nums[i] as i64 {
                return prefix_sum + nums[i] as i64;
            }
            prefix_sum -= nums[i - 1] as i64;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 5]), 15);
        assert_eq!(Solution::largest_perimeter(vec![1, 12, 1, 2, 5, 50, 3]), 12);
        assert_eq!(Solution::largest_perimeter(vec![5, 5, 50]), -1);
    }
}
