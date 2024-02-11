pub struct Solution;

impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dp = vec![vec![vec![-1; m]; m]; n];

        dfs(0, 0, m - 1, &grid, &mut dp)
    }
}

fn dfs(i: usize, r1: usize, r2: usize, map: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
    let n = map.len();
    let m = map[0].len();

    if dp[i][r1][r2] != -1 {
        return dp[i][r1][r2];
    }

    if i == n - 1 {
        return if r1 == r2 {
            map[i][r1]
        } else {
            map[i][r1] + map[i][r2]
        };
    }

    let mut cherries = 0;

    (-1..=1).for_each(|d1| {
        (-1..=1)
            .map(|d2| (r1 as i32 + d1, r2 as i32 + d2))
            .filter(|&(new_r1, new_r2)| {
                new_r1 >= 0 && new_r1 < m as i32 && new_r2 >= 0 && new_r2 < m as i32
            })
            .map(|(new_r1, new_r2)| (new_r1 as usize, new_r2 as usize))
            .for_each(|(new_r1, new_r2)| {
                let next = dfs(i + 1, new_r1, new_r2, map, dp);

                if r1 == r2 {
                    cherries = cherries.max(next + map[i][r1]);
                } else {
                    cherries = cherries.max(next + map[i][r1] + map[i][r2]);
                }
            });
    });

    dp[i][r1][r2] = cherries;
    cherries
}
