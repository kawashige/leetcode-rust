pub struct Solution {}

impl Solution {
    pub fn count(s: &str) -> [i32; 26] {
        s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        })
    }

    pub fn min_steps(s: String, t: String) -> i32 {
        let s_count = Self::count(&s);
        let t_count = Self::count(&t);

        s_count
            .iter()
            .zip(t_count.iter())
            .map(|(s, t)| (s - t).abs())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2186() {
        assert_eq!(
            Solution::min_steps("leetcode".to_string(), "coats".to_string()),
            7
        );
        assert_eq!(
            Solution::min_steps("night".to_string(), "thing".to_string()),
            0
        );
    }
}
