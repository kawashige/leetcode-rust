pub struct Solution {}

impl Solution {
    pub fn non_special_count(l: i32, r: i32) -> i32 {
        let max = (r as f64).sqrt() as usize;
        let mut is_prime = vec![true; max + 1];
        for i in 2..is_prime.len() {
            if is_prime.len() < i * i {
                break;
            }
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let min = ((l as f64).sqrt() as i32..=l).find(|i| l <= i * i).unwrap() as usize;
        r - l + 1 - is_prime[min.max(2)..].iter().filter(|x| **x).count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3233() {
        assert_eq!(Solution::non_special_count(5, 7), 3);
        assert_eq!(Solution::non_special_count(4, 16), 11);
    }
}
