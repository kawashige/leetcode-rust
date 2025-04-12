use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn calc_count(counts: &[i32], n: i32, factorial: &Vec<i64>) -> i64 {
        let mut count = factorial[n as usize];
        for i in 0..counts.len() {
            if 1 < counts[i] {
                count /= factorial[counts[i] as usize];
            }
        }
        return count;
    }
    pub fn recurse(
        digits: &mut Vec<i32>,
        n: i32,
        k: i32,
        result: &mut i64,
        factorial: &Vec<i64>,
        seen: &mut HashSet<String>,
    ) {
        if digits.len() == (n as usize + 1) / 2 {
            let mut num = 0;
            let mut counts = [0; 10];
            for i in 0..digits.len() {
                num = num * 10 + digits[i] as i64;
                counts[digits[i] as usize] += 1;
            }
            for i in (0..digits.len() - if n % 2 == 0 { 0 } else { 1 }).rev() {
                num = num * 10 + digits[i] as i64;
                counts[digits[i] as usize] += 1;
            }

            if num % k as i64 == 0 {
                let key = counts
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",");
                if seen.contains(&key) {
                    return;
                }
                seen.insert(key);
                let mut count = Self::calc_count(&counts, n, factorial);
                if 0 < counts[0] {
                    counts[0] -= 1;
                    count -= Self::calc_count(&counts, n - 1, factorial);
                }
                *result += count;
            }
            return;
        }

        for d in if digits.is_empty() { 1 } else { 0 }..10 {
            digits.push(d);
            Self::recurse(digits, n, k, result, factorial, seen);
            digits.pop();
        }
    }
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let mut result = 0;
        let mut factorial = vec![1; 11];
        for i in 2..factorial.len() {
            factorial[i] = factorial[i - 1] * i as i64;
        }
        Self::recurse(
            &mut Vec::new(),
            n,
            k,
            &mut result,
            &factorial,
            &mut HashSet::new(),
        );
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3272() {
        assert_eq!(Solution::count_good_integers(3, 5), 27);
        assert_eq!(Solution::count_good_integers(1, 4), 2);
        assert_eq!(Solution::count_good_integers(5, 6), 2468);
    }
}
