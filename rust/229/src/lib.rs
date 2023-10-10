pub struct Solution;

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> Vec<i32> {
        let mut curr;
        let mut cnt = 0;
        let n = nums.len() / 3;
        let mut res = Vec::with_capacity(n);

        if nums.is_empty() {
            return res;
        }

        // Sort the array first
        nums.sort();
        curr = nums[0];

        for i in nums {
            if curr == i {
                cnt += 1;
            } else {
                if cnt > n {
                    res.push(curr);
                }

                curr = i;
                cnt = 1;
            }
        }

        // Handle the last element
        if cnt > n {
            res.push(curr);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 2, 3];
        let res = vec![3];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![1, 1, 1, 3, 3, 2, 2, 2];
        let res = vec![1, 2];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![1, 2, 3, 4];
        let res = vec![];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![1];
        let res = vec![1];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![];
        let res = vec![];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
        let res = vec![];
        assert_eq!(Solution::majority_element(nums), res);

        let nums = vec![1, 1, 1, 2, 2, 2, 3, 3];
        let res = vec![1, 2];
        assert_eq!(Solution::majority_element(nums), res);
    }
}