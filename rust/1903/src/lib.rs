pub struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        num.rfind(|c| (c as u8) & 1 == 1)
            .map(|i| &num[..=i])
            .unwrap_or_default()
            .into()
    }
}
