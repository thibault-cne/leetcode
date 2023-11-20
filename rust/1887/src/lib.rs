pub struct Solution;

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let mut ans = 0;
        let mut prev = 0;
        
        for i in 1..nums.len() {
            if nums[i] != nums[i-1] {
                prev += 1;
            }
            ans += prev;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1,2,3,4,4,5,5,5];
        let ans = 21;
        assert_eq!(Solution::reduction_operations(nums), ans);
    }
}