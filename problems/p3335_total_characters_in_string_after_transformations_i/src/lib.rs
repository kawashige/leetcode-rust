pub struct Solution {}

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut count = s.as_bytes().iter().fold([0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        for _ in 0..t {
            let mut next_count = [0; 26];
            for i in 0..25 {
                next_count[i + 1] = count[i];
            }
            next_count[0] = count[25];
            next_count[1] = (count[25] + count[0]) % M;
            count = next_count;
        }
        count.into_iter().fold(0, |s, c| (s + c) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3335() {
        assert_eq!(
            Solution::length_after_transformations("abcyy".to_string(), 2),
            7
        );
        assert_eq!(
            Solution::length_after_transformations("azbk".to_string(), 1),
            5
        );
    }
}
