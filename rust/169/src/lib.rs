pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut res, mut majority) = (0, 0);

        nums.iter().for_each(|&num| {
            if majority == 0 {
                res = num;
                majority = 1;
            } else if res == num {
                majority += 1;
            } else {
                majority -= 1;
            }
        });

        res
    }
}
