pub struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let freq = |s: String| {
            let mut m = [0; 26];
            s.bytes().for_each(|b| {
                let i = (b - b'a') as usize;
                *unsafe { m.get_unchecked_mut(i) } += 1;
            });
            m
        };

        let s_cpt = freq(s);
        let t_cpt = freq(t);

        s_cpt
            .iter()
            .zip(t_cpt)
            .fold(0, |acc, (&a, b)| if a <= b { acc } else { acc + (a - b) })
    }
}
