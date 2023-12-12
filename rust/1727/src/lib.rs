pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let mut max = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if i > 0 && matrix[i][j] == 1 {
                    matrix[i][j] += matrix[i - 1][j];
                }
            }
        }

        for i in 0..matrix.len() {
            matrix[i].sort_unstable_by(|a, b| b.cmp(a));
            for j in 0..matrix[0].len() {
                max = max.max(matrix[i][j] * (j as i32 + 1));
            }
        }

        max
    }
}