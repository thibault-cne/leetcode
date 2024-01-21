pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut rob = 0;
        let mut norob = 0;

        nums.into_iter().for_each(|e| {
            let t = norob;
            norob = rob.max(norob);
            rob = t + e;
        });

        rob.max(norob)
    }
}
