pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows = vec![vec![1]];

        (2..num_rows as usize + 1).for_each(|i| {
            let mut row = vec![1];
            let prev_row = rows.last().unwrap();
            let mut index = 1;

            loop {
                if row.len() == i - 1 {
                    break;
                }

                let push = prev_row.get(index - 1).unwrap() + prev_row.get(index).unwrap();
                row.push(push);
                index += 1;
            }

            row.push(1);
            rows.push(row);
        });

        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expect = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];

        assert_eq!(Solution::generate(5), expect);

        let expect = vec![vec![1]];

        assert_eq!(Solution::generate(1), expect);

        let expect = vec![vec![1], vec![1, 1]];

        assert_eq!(Solution::generate(2), expect);

        let expect = vec![vec![1], vec![1, 1], vec![1, 2, 1]];

        assert_eq!(Solution::generate(3), expect);
    }
}
