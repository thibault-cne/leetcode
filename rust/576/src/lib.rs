pub struct Solution;

const DIRS: &[i64; 5] = &[-1, 0, 1, 0, -1];
const MODULO: i64 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, i: i32, j: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut prev: Vec<Vec<i64>> = vec![vec![0; n]; m];
        let mut res = 0;
        prev[i as usize][j as usize] = 1;

        (0..max_move).for_each(|_| {
            let mut curr: Vec<Vec<i64>> = vec![vec![0; n]; m];
            (0..m).for_each(|j| {
                (0..n).for_each(|i| {
                    if prev[j][i] != 0 {
                        (0..4).for_each(|i_| {
                            let (x, y) = (
                                (i as i64 + DIRS[i_ + 1]) as usize,
                                (j as i64 + DIRS[i_]) as usize,
                            );

                            if x >= n || y >= m {
                                res = (res + prev[j][i]) % MODULO;
                            } else {
                                curr[y][x] = (curr[y][x] + prev[j][i]) % MODULO;
                            }
                        });
                    }
                });
            });

            prev = curr;
        });

        res as _
    }
}
