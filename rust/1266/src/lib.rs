pub struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..(points.len() - 1) {
            let x = points[i][0] - points[i + 1][0];
            let y = points[i][1] - points[i + 1][1];
            ans += std::cmp::max(x.abs(), y.abs());
        }

        ans
    }
}
