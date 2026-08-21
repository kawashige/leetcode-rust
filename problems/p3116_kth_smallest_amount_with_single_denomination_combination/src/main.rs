pub struct Solution {}

impl Solution {
    pub fn gcd(m: i64, n: i64) -> i64 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn lcm(a: i64, b: i64) -> i64 {
        a / Self::gcd(a, b) * b
    }

    fn count(x: i64, coins: &[i64]) -> i64 {
        let n = coins.len();
        let mut total = 0i64;

        for mask in 1usize..(1usize << n) {
            let mut lcm = 1i64;
            let mut bits = 0;

            for i in 0..n {
                if mask & (1usize << i) == 0 {
                    continue;
                }

                bits += 1;

                let coin = coins[i] as i64;
                let g = Self::gcd(lcm, coin);

                // lcm / g * coin > x なら寄与は0
                if lcm / g > x / coin {
                    lcm = x + 1;
                    break;
                }

                lcm = lcm / g * coin;
            }

            if lcm <= x {
                if bits % 2 == 1 {
                    total += x / lcm;
                } else {
                    total -= x / lcm;
                }
            }
        }

        total
    }

    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        let mut coins = coins;
        coins.sort_unstable();
        let mut new_coins = Vec::new();
        for i in 0..coins.len() {
            let mut is_ok = true;
            for j in 0..i {
                if coins[i] % coins[j] == 0 {
                    is_ok = false;
                    break;
                }
            }
            if is_ok {
                new_coins.push(coins[i] as i64);
            }
        }

        let k = k as i64;
        let mut lcms = vec![vec![0; new_coins.len()]; new_coins.len()];
        for i in 0..new_coins.len() {
            for j in 0..i {
                lcms[i][j] = Self::lcm(new_coins[i], new_coins[j]) as i64;
            }
        }

        let mut ng = 0;
        let mut ok = 25 * k;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if k <= Self::count(mid, &new_coins) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3116() {
        assert_eq!(Solution::find_kth_smallest(vec![3, 6, 9], 3), 9);
        assert_eq!(Solution::find_kth_smallest(vec![5, 2], 7), 12);
    }
}

fn main() {}
