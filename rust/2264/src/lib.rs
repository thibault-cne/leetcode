pub struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let n = num
            .as_bytes()
            .windows(3)
            .filter(|s| s[0] == s[1] && s[1] == s[2])
            .flat_map(|s| String::from_utf8(s.to_vec()).unwrap().parse::<i32>())
            .fold(i32::MIN, |acc, v| if v > acc { v } else { acc });

        if n == i32::MIN {
            "".to_string()
        } else {
            format!("{:03}", n)
        }
    }
}
