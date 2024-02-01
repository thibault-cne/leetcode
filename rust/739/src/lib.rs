pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        let mut result = vec![0; temperatures.len()];

        temperatures.iter().enumerate().rev().for_each(|(i, e)| {
            while let Some(&j) = stack.last() {
                if temperatures[j] > *e {
                    result[i] = (j - i) as i32;
                    break;
                } else {
                    stack.pop();
                }
            }
            stack.push(i);
        });

        result
    }
}
