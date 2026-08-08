pub struct Solution {}

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        let mut temp = t;
        for i in 2..=9 {
            while temp % i == 0 {
                temp /= i;
            }
        }
        if temp > 1 {
            return "-1".to_string();
        }

        let n = num.len();
        let mut rem = vec![0i64; n + 1];
        rem[0] = t;
        let mut pos = n - 1;

        let mut num_chars: Vec<char> = num.chars().collect();

        for i in 0..n {
            if num_chars[i] == '0' {
                pos = i;
                break;
            }
            rem[i + 1] = rem[i] / gcd(rem[i], (num_chars[i] as u8 - b'0') as i64);
        }

        if rem[n] == 1 {
            return num;
        }

        for i in (0..=pos).rev() {
            loop {
                num_chars[i] = ((num_chars[i] as u8) + 1) as char;
                if num_chars[i] > '9' {
                    break;
                }

                let mut t_now = rem[i] / gcd(rem[i], (num_chars[i] as u8 - b'0') as i64);
                let mut k = 9i64;

                for j in (i + 1..n).rev() {
                    while t_now % k != 0 {
                        k -= 1;
                    }
                    t_now /= k;
                    num_chars[j] = (b'0' + k as u8) as char;
                }

                if t_now == 1 {
                    return num_chars.iter().collect();
                }
            }
        }

        let mut ans = Vec::new();
        let mut original_t = t;

        for i in (2..=9).rev() {
            while original_t % i == 0 {
                ans.push((b'0' + i as u8) as char);
                original_t /= i;
            }
        }

        let padding = std::cmp::max(n as i32 + 1 - ans.len() as i32, 0) as usize;
        for _ in 0..padding {
            ans.push('1');
        }

        ans.reverse();
        ans.iter().collect()
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3348() {
        assert_eq!(
            Solution::smallest_number("1234".to_string(), 256),
            "1488".to_string()
        );
        assert_eq!(
            Solution::smallest_number("12355".to_string(), 50),
            "12355".to_string()
        );
        assert_eq!(
            Solution::smallest_number("11111".to_string(), 26),
            "-1".to_string()
        );
    }
}

fn main() {}
