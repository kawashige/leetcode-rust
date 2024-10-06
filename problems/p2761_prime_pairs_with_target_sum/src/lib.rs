pub struct Solution {}

impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let mut is_prime = vec![true; n as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let mut result = Vec::new();
        for i in 2..is_prime.len() {
            if n as usize - i < i {
                break;
            }
            if is_prime[i] && is_prime[n as usize - i] {
                result.push(vec![i as i32, n - i as i32]);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2761() {
        assert_eq!(Solution::find_prime_pairs(10), vec![vec![3, 7], vec![5, 5]]);
        assert_eq!(Solution::find_prime_pairs(2), vec![] as Vec<Vec<i32>>);
    }
}
