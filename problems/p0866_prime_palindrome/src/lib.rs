pub struct Solution {}

impl Solution {
    pub fn check_prime(x: usize, primes: &Vec<usize>) -> bool {
        if x < 2 {
            return false;
        }
        let mut is_prime = true;
        for i in 0..primes.len() {
            if x <= primes[i] {
                break;
            }
            if x % primes[i] == 0 {
                is_prime = false;
            }
        }
        is_prime
    }

    pub fn prime_palindrome(n: i32) -> i32 {
        let max = 2 * 100_000_000;
        let mut is_prime = vec![true; (max as f64).sqrt() as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..is_prime.len() {
            if is_prime[i] {
                continue;
            }

            for j in ((i + i)..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }

        let primes = (0..is_prime.len())
            .filter(|i| is_prime[*i])
            .collect::<Vec<usize>>();

        let n = n as usize;
        let mut l = n.to_string().len();
        loop {
            if l == 1 {
                for x in 1..10 {
                    if x >= n && Self::check_prime(x, &primes) {
                        return x as i32;
                    }
                }
            } else if l == 2 {
                for x in 1..10 {
                    let num = format!("{}{}", x, x.to_string().chars().rev().collect::<String>())
                        .parse::<usize>()
                        .unwrap();
                    if num >= n && Self::check_prime(num, &primes) {
                        return num as i32;
                    }
                }
            } else if l % 2 == 1 {
                for x in 10_usize.pow(l as u32 / 2 - 1)..10_usize.pow(l as u32 / 2) {
                    for y in 0..10 {
                        let num = format!(
                            "{}{}{}",
                            x,
                            y,
                            x.to_string().chars().rev().collect::<String>()
                        )
                        .parse::<usize>()
                        .unwrap();
                        if num >= n && Self::check_prime(num, &primes) {
                            return num as i32;
                        }
                    }
                }
            }

            l += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0866() {
        assert_eq!(Solution::prime_palindrome(100000000), 100030001);
        assert_eq!(Solution::prime_palindrome(45887963), 100030001);
        assert_eq!(Solution::prime_palindrome(6), 7);
        assert_eq!(Solution::prime_palindrome(8), 11);
        assert_eq!(Solution::prime_palindrome(13), 101);
    }
}
