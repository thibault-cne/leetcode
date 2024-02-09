pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; (n + 1) as usize];

        (1..=n).for_each(|i| {
            if i % 2 == 0 {
                res[i as usize] = res[(i / 2) as usize];
            } else {
                res[i as usize] = res[(i - 1) as usize / 2] + 1;
            }
        });

        res
    }
}
