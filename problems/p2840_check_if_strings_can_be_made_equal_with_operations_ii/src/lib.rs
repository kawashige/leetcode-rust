pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [[i32; 26]; 2] {
        s.as_bytes()
            .iter()
            .enumerate()
            .fold([[0; 26]; 2], |mut count, (i, b)| {
                count[i % 2][(b - b'a') as usize] += 1;
                count
            })
    }

    pub fn check_strings(s1: String, s2: String) -> bool {
        let c1 = Self::count(&s1);
        let c2 = Self::count(&s2);

        for i in 0..c1.len() {
            for j in 0..c1[0].len() {
                if c1[i][j] != c2[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2840() {
        assert!(Solution::check_strings(
            "abcdba".to_string(),
            "cabdab".to_string()
        ));
        assert!(!Solution::check_strings(
            "abe".to_string(),
            "bea".to_string()
        ));
    }
}
