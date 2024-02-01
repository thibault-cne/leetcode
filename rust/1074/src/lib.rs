pub struct Solution;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let row_count = matrix.len();
        let col_count = matrix[0].len();

        (0..row_count).fold(0, |mut acc, l| {
            let mut temp = vec![0; col_count];

            (l..row_count).for_each(|r| {
                let mut sum = 0;
                let mut map = std::collections::HashMap::new();
                map.insert(0, 1);

                (0..col_count).for_each(|c| {
                    temp[c] += matrix[r][c];
                    sum += temp[c];
                    acc += map.get(&(sum - target)).unwrap_or(&0);
                    *map.entry(sum).or_insert(0) += 1;
                });
            });

            acc
        })
    }
}
