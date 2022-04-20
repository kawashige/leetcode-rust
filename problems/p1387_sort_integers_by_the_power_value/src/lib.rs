pub struct Solution {}

impl Solution {
    pub fn calc_power(n: usize, powers: &mut Vec<i32>) -> i32 {
        if powers[n] == -1 {
            let next = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
            powers[n] = Self::calc_power(next, powers) + 1;
        }
        powers[n]
    }

    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut powers = vec![-1; 300000];
        powers[1] = 0;
        let mut power_values = (lo as usize..=hi as usize)
            .map(|i| (Self::calc_power(i, &mut powers), i))
            .collect::<Vec<_>>();
        power_values.sort_unstable();
        power_values[k as usize - 1].1 as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1387() {
        assert_eq!(Solution::get_kth(1, 1000, 2), 2);
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
