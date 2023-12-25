pub struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        points.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
        let expected = 1;
        assert_eq!(Solution::max_width_of_vertical_area(points), expected);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3, 1], vec![9, 0], vec![1, 0], vec![1, 4], vec![5, 3], vec![8, 8]];
        let expected = 3;
        assert_eq!(Solution::max_width_of_vertical_area(points), expected);
    }
}