pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = std::collections::HashMap::new();

        matrix[0]
            .iter()
            .enumerate()
            .map(|(i, _)| Solution::memoization(&matrix, &mut dp, (0, i)))
            .min()
            .unwrap()
    }

    fn memoization(
        matrix: &[Vec<i32>],
        dp: &mut std::collections::HashMap<(usize, usize), i32>,
        point: (usize, usize),
    ) -> i32 {
        if let Some(&res) = dp.get(&point) {
            return res;
        }

        let n = matrix.len();
        if point.0 == (n - 1) {
            return matrix[point.0][point.1];
        }

        let min = (point.1.saturating_sub(1)..=(point.1 + 1))
            .filter(|&p| p < n)
            .map(|p| Solution::memoization(matrix, dp, (point.0 + 1, p)))
            .min()
            .unwrap();

        dp.insert(point, matrix[point.0][point.1] + min);
        matrix[point.0][point.1] + min
    }
}
