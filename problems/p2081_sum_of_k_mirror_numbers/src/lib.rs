pub struct Solution {}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let k = k as i64;
        let mut nums = Vec::new();

        let mut i = 1;
        let mut prev = 0;
        loop {
            let digits = i
                .to_string()
                .as_bytes()
                .iter()
                .map(|x| (x - b'0') as i64)
                .collect::<Vec<_>>();
            if prev != digits.len() && (n as usize) <= nums.len() {
                break;
            }
            prev = digits.len();

            // digits.len() * 2 - 1
            println!("digits: {:?}", digits);
            let num = digits
                .iter()
                .chain(digits[..digits.len() - 1].iter().rev())
                .fold(0, |acc, x| acc * 10 + *x as i64);
            let mut remains = num;
            let mut k_digits = Vec::new();
            while 0 < remains {
                k_digits.push(remains % k);
                remains /= k;
            }
            if (0..k_digits.len() / 2).all(|i| k_digits[i] == k_digits[k_digits.len() - 1 - i]) {
                nums.push(num);
            }

            // digits.len() * 2
            let num = digits
                .iter()
                .chain(digits.iter().rev())
                .fold(0, |acc, x| acc * 10 + *x as i64);
            let mut remains = num;
            let mut k_digits = Vec::new();
            while 0 < remains {
                k_digits.push(remains % k);
                remains /= k;
            }
            if (0..k_digits.len() / 2).all(|i| k_digits[i] == k_digits[k_digits.len() - 1 - i]) {
                nums.push(num);
            }

            i += 1;
        }

        nums.sort_unstable();
        nums.into_iter().take(n as usize).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2081() {
        assert_eq!(Solution::k_mirror(2, 5), 25);
        assert_eq!(Solution::k_mirror(3, 7), 499);
        assert_eq!(Solution::k_mirror(7, 17), 20379000);
    }
}
