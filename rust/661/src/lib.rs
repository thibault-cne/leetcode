pub struct Solution;

macro_rules! avg {
    ($coord:expr, $img:expr) => {
        {
            let i_start = $coord.0.saturating_sub(1);
            let i_end = ($coord.0 + 1).min($img.len() - 1);
            let j_start = $coord.1.saturating_sub(1);
            let j_end = ($coord.1 + 1).min($img[0].len() - 1);

            (i_start..=i_end)
                .map(|i| (j_start..=j_end).map(|j| $img[i][j]).sum::<i32>())
                .sum::<i32>()
                / ((i_end - i_start + 1) * (j_end - j_start + 1)) as i32
        }
    };
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = img.len();
        let n = img[0].len();
        let mut res = vec![vec![0; n]; m];

        (0..m).for_each(|i| (0..n).for_each(|j| res[i][j] = avg!((i, j), &img)));

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1],]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0],]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100],
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137],
            ]
        );
    }
}
