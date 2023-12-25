pub struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut diff = vec![vec![0; n]; m];
        let mut cache = std::collections::HashMap::<String, i32>::new();

        (0..m).for_each(|i| {
            (0..n).for_each(|j| {
                let row_i = match (
                    cache.get(&format!("one_row_{i}")),
                    cache.get(&format!("zero_row_{i}")),
                ) {
                    (Some(one), Some(zero)) => (*one, *zero),
                    (None, None) => {
                        let row_i = grid[i].iter().fold((0, 0), |acc, v| {
                            if *v == 1 {
                                (acc.0 + 1, acc.1)
                            } else {
                                (acc.0, acc.1 + 1)
                            }
                        });
                        cache.insert(format!("one_row_{i}"), row_i.0);
                        cache.insert(format!("zero_row_{i}"), row_i.1);
                        row_i
                    }
                    (_, _) => unreachable!(),
                };
                let col_j = match (
                    cache.get(&format!("one_col_{j}")),
                    cache.get(&format!("zero_col_{j}")),
                ) {
                    (Some(one), Some(zero)) => (*one, *zero),
                    (None, None) => {
                        let col_j = grid.iter().map(|row| row[j]).fold((0, 0), |acc, v| {
                            if v == 1 {
                                (acc.0 + 1, acc.1)
                            } else {
                                (acc.0, acc.1 + 1)
                            }
                        });
                        cache.insert(format!("one_col_{j}"), col_j.0);
                        cache.insert(format!("zero_col_{j}"), col_j.1);
                        col_j
                    }
                    (_, _) => unreachable!(),
                };

                diff[i][j] = row_i.0 + col_j.0 - row_i.1 - col_j.1;
            });
        });

        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]]),
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
        );
    }
}
