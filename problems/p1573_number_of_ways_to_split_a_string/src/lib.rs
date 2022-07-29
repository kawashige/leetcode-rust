pub struct Solution {}

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        const M: usize = 1_000_000_007;

        let ones_count = s.as_bytes().iter().filter(|b| b == &&b'1').count();

        if ones_count == 0 {
            let l = s.len();
            return (((l - 1) * (l - 2) / 2) % M) as i32;
        } else if ones_count % 3 != 0 {
            return 0;
        }

        let mut i = 0;
        let mut count = 0;
        while count < ones_count / 3 {
            if s.as_bytes()[i] == b'1' {
                count += 1;
            }
            i += 1;
        }
        let left = (i..s.len())
            .take_while(|j| s.as_bytes()[*j] == b'0')
            .count();

        let mut i = s.len() - 1;
        let mut count = 0;
        while count < ones_count / 3 {
            if s.as_bytes()[i] == b'1' {
                count += 1;
            }
            i -= 1;
        }
        let right = (0..=i)
            .rev()
            .take_while(|j| s.as_bytes()[*j] == b'0')
            .count();

        (((left + 1) * (right + 1)) % M) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1573() {
        assert_eq!(Solution::num_ways("10101".to_string()), 4);
        assert_eq!(Solution::num_ways("1001".to_string()), 0);
        assert_eq!(Solution::num_ways("0000".to_string()), 3);
    }
}
