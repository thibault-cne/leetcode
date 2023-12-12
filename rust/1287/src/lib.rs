pub struct Solution;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut cpt = 1;

        for (i, &e) in arr.iter().enumerate() {
            if i == 0 {
                continue;
            }

            if e != arr[i - 1] && (arr.len() / 4) < cpt {
                return arr[i - 1];
            } else if e != arr[i - 1] {
                cpt = 1;
            } else {
                cpt += 1;
            }
        }

        arr[arr.len() - 1]
    }
}
