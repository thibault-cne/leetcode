pub struct Solution;

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; nums.len()];

        nums.iter()
            .filter(|e| **e > 0)
            .enumerate()
            .for_each(|(i, e)| {
                res[i * 2] = *e;
            });

        nums.iter()
            .filter(|e| **e < 0)
            .enumerate()
            .for_each(|(i, e)| {
                res[i * 2 + 1] = *e;
            });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
    }
}
