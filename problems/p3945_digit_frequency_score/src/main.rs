pub struct Solution {}

impl Solution {
    pub fn digit_frequency_score(n: i32) -> i32 {
        let mut freq = [0; 10];
        let mut n = n as usize;
        while 0 < n {
            freq[n % 10] += 1;
            n /= 10;
        }
        (0..freq.len()).map(|i| i * freq[i]).sum::<usize>() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3945() {
        assert_eq!(Solution::digit_frequency_score(122), 5);
        assert_eq!(Solution::digit_frequency_score(101), 2);
    }
}

fn main() {}
