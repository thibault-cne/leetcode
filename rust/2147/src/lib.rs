pub struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        if corridor.len() < 4 {
            return 0;
        }

        const MOD: i32 = 1e9 as i32 + 7;
        let mut ans = 1;
        let mut flower_cpt = 0;

        let mut iter = corridor.chars();

        if !skip_while(&mut iter, |&c| c != 'S') {
            return 0;
        }
        if !skip_while(&mut iter, |&c| c != 'S') {
            return 0;
        }

        while let Some(c) = iter.next() {
            match c {
                'S' => {
                    let stmt = skip_while(&mut iter, |&c| c != 'S');
                    if stmt {
                        println!("flower_cpt: {}", flower_cpt);
                        ans = (ans * (flower_cpt + 1)) % MOD;
                        flower_cpt = 0;
                    } else {
                        return 0;
                    }
                }
                'P' => flower_cpt += 1,
                _ => unreachable!(),
            }
        }

        ans
    }
}

pub fn skip_while<I, T, P>(i: &mut I, mut predicate: P) -> bool
where
    I: Iterator<Item = T>,
    P: FnMut(&T) -> bool,
{
    loop {
        match i.next() {
            Some(x) => {
                if !predicate(&x) {
                    break true;
                }
            }
            None => break false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
    }
}