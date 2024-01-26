pub struct Solution;

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let masks: Vec<u32> = arr
            .iter()
            .map(|s| {
                (
                    s.bytes().map(|b| 1 << (b - b'a')).sum::<u32>(),
                    s.len() as u32,
                )
            })
            .filter(|(m, l)| m.count_ones() == *l)
            .map(|(m, _)| m)
            .collect();

        let mut concats = vec![0];
        let mut ans = 0;

        for m in masks.iter().skip(1) {
            for i in 0..concats.len() {
                if m & concats[i] == 0 {
                    let cc = m | concats[i];
                    concats.push(cc);
                    ans = ans.max(cc.count_ones());
                }
            }
        }

        ans as i32
    }
}
