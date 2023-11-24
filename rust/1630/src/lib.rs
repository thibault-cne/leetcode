pub struct Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![true; l.len()];

        for i in 0..l.len() {
            let mut slice: Vec<i32> = nums[l[i] as usize..r[i] as usize].to_vec();

            if slice.len() == 1 {
                continue;
            }

            slice.sort_unstable();

            let q = slice[1] - slice[0];
            for j in 1..slice.len() {
                if slice[j] - slice[j - 1] != q {
                    ans[i] = false;
                    break;
                }
            }
        }

        ans
    }
}
