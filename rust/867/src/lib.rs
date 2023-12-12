pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let n = matrix.len();
        let m = matrix[0].len();

        (0..m).for_each(|i| {
            let mut temp = Vec::new();
            (0..n).for_each(|j| {
                temp.push(matrix[j][i]);
            });
            res.push(temp);
        });

        res
    }
}
