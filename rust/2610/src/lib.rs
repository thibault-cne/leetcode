pub struct Solution;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut table = std::collections::HashMap::new();

        nums.iter().for_each(|n| *table.entry(n).or_insert(0) += 1);

        let max = *table.values().max().unwrap() as usize;
        let mut res = Vec::new();

        (0..max).for_each(|_| {
            let mut temp = Vec::new();
            table
                .iter_mut()
                .filter(|(_, v)| **v > 0)
                .for_each(|(k, v)| {
                    temp.push(**k);
                    *v -= 1;
                });
            res.push(temp);
        });

        res
    }
}
