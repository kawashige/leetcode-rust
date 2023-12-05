pub struct Solution {}

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_s = num.to_string();
        let mut count = 0;

        for i in 0..=num_s.len() - k as usize {
            let n = num_s[i..i + k as usize].parse::<i32>().unwrap();
            if 0 < n && num % n == 0 {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2269() {
        assert_eq!(Solution::divisor_substrings(240, 2), 2);
        assert_eq!(Solution::divisor_substrings(430043, 2), 2);
    }
}
