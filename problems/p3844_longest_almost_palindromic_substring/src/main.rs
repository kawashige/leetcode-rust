pub struct Solution {}

impl Solution {
    pub fn expand(l: i32, r: i32, s: &str) -> (i32, i32) {
        let (mut l, mut r) = (l, r);
        while 0 <= l && r < s.len() as i32 && s.as_bytes()[l as usize] == s.as_bytes()[r as usize] {
            l -= 1;
            r += 1;
        }
        (l, r)
    }
    pub fn almost_palindromic(s: String) -> i32 {
        if s.len() <= 2 {
            return 2;
        }
        let mut result = 0;

        for i in 0..s.len() as i32 {
            for (l, r) in [(i, i), (i, i + 1)].iter() {
                let (l1, r1) = Self::expand(*l, *r, &s);
                result = result.max(r1 - l1);

                let (l2, r2) = Self::expand(l1 - 1, r1, &s);
                result = result.max(r2 - l2 - 1);

                let (l3, r3) = Self::expand(l1, r1 + 1, &s);
                result = result.max(r3 - l3 - 1);

                result = result.min(s.len() as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3844() {
        assert_eq!(Solution::almost_palindromic("abc".to_string()), 2);
        assert_eq!(Solution::almost_palindromic("abca".to_string()), 4);
        assert_eq!(Solution::almost_palindromic("abba".to_string()), 4);
        assert_eq!(Solution::almost_palindromic("zzabba".to_string()), 5);
    }
}

fn main() {}
