pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();

        (0..n)
            .filter(|&j| mat.iter().map(|row| row[j]).sum::<i32>() == 1)
            .map(|j| mat.iter().position(|row| row[j] == 1).unwrap())
            .filter(|&i| mat[i].iter().copied().sum::<i32>() == 1)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            1
        );
        assert_eq!(
            Solution::num_special(vec![
                vec![0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 1, 0, 0, 1],
                vec![0, 0, 0, 0, 1, 0, 0, 0],
                vec![1, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0]
            ]),
            1
        );
    }
}
