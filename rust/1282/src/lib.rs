use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cache: HashMap<u16, Vec<i32>> = HashMap::new();
        let mut vec: Vec<Vec<i32>> = Vec::new();

        group_sizes.iter().enumerate().for_each(|(i, &s)| {
            let group = cache
                .entry(s as u16)
                .or_insert(Vec::with_capacity(s as usize));

            group.push(i as i32);

            if group.len() == s as usize {
                let group = group.clone();
                cache.remove(&(s as u16));
                vec.push(group);
            }
        });

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let group_sizes = vec![3, 3, 3, 3, 3, 1, 3];
        let expected = vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]];

        expected
            .iter()
            .for_each(|v| assert!(Solution::group_the_people(group_sizes.clone()).contains(v)));
    
        let group_sizes = vec![2, 1, 3, 3, 3, 2];
        let expected = vec![vec![1], vec![0, 5], vec![2, 3, 4]];

        expected
            .iter()
            .for_each(|v| assert!(Solution::group_the_people(group_sizes.clone()).contains(v)));

        let group_sizes = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let expected = vec![vec![0], vec![1], vec![2], vec![3], vec![4], vec![5], vec![6], vec![7]];

        expected
            .iter()
            .for_each(|v| assert!(Solution::group_the_people(group_sizes.clone()).contains(v)));
    }
}
