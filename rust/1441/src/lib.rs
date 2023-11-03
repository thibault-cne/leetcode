pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut res = Vec::new();

        for i in 1..=n {
            if i == target[target.len() - 1] {
                res.push("Push".to_string());
                break;
            }

            if target.contains(&i) {
                res.push("Push".to_string());
            } else {
                res.push("Push".to_string());
                res.push("Pop".to_string());
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec!["Push", "Push", "Pop", "Push"]
        );

        assert_eq!(
            Solution::build_array(vec![1, 2, 3], 3),
            vec!["Push", "Push", "Push"]
        );

        assert_eq!(
            Solution::build_array(vec![1, 2], 4),
            vec!["Push", "Push"]
        );

        assert_eq!(
            Solution::build_array(vec![2, 3, 4], 4),
            vec!["Push", "Pop", "Push", "Push", "Push"]
        );

        assert_eq!(
            Solution::build_array(vec![3, 6], 6),
            vec!["Push", "Pop", "Push", "Pop", "Push", "Push", "Pop", "Push", "Pop", "Push"]
        );
    }
}
