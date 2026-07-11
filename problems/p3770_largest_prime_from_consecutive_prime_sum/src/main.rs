use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn largest_prime(n: i32) -> i32 {
        if n == 2 {
            return 2;
        } else if n == 1 {
            return 0;
        }

        let n = n as usize;
        let mut is_prime = vec![true; n + 1];
        let mut set = HashSet::new();
        let mut result = 2;
        let mut sum = 0;

        for i in 2..=n {
            if !is_prime[i] {
                continue;
            }
            if set.contains(&i) {
                result = i;
            }
            sum += i;
            set.insert(sum);

            for j in (i + i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3370() {
        assert_eq!(Solution::largest_prime(20), 17);
        assert_eq!(Solution::largest_prime(2), 2);
    }
}

fn main() {}
