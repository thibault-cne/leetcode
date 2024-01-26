pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = std::collections::HashSet::new();
        let mut dedup = 0;

        for e in nums.iter() {
            if !set.insert(e) {
                dedup = *e;
                break;
            }
        }

        let n = nums.len() as i32;
        let expected_sum = n * (n + 1) / 2;
        let missing_number = expected_sum - (nums.iter().sum::<i32>() - dedup);
        vec![dedup, missing_number]
    }
}
