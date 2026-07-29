pub struct Solution {}

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let partition = s.len() / 2;
        let mut bucket = [0_i32; 26];
        let s_bytes = s.as_bytes();

        for i in 0..partition {
            bucket[(s_bytes[i] - b'a') as usize] += 1;
        }

        let comb = |n: i32, mut m: i32, k_val: i32| -> i64 {
            let mut res = 1_i64;
            if n - m < m {
                m = n - m;
            }

            for i in 1..=m {
                res = res * (n as i64 - i as i64 + 1) / (i as i64);
                if res > k_val as i64 {
                    return (k_val + 1) as i64;
                }
            }
            res
        };

        let mut left = String::with_capacity(partition);
        let mut start_index = 1_i64;
        let k_i64 = k as i64;

        for pos in 0..partition {
            for i in 0..26 {
                if bucket[i] == 0 {
                    continue;
                }

                bucket[i] -= 1;

                let mut ways = 1_i64;
                let mut rem = (partition - pos - 1) as i32;

                for j in 0..26 {
                    if bucket[j] == 0 {
                        continue;
                    }

                    ways *= comb(rem, bucket[j], k);
                    if ways > k_i64 {
                        break;
                    }
                    rem -= bucket[j];
                }

                if start_index + ways > k_i64 {
                    left.push((i as u8 + b'a') as char);
                    break;
                }

                bucket[i] += 1;
                start_index += ways;
            }
        }

        if left.len() < partition {
            return String::new();
        }

        let total_len = s.len();
        let mut res = vec![0_u8; total_len];
        let left_bytes = left.as_bytes();

        for i in 0..partition {
            res[i] = left_bytes[i];
            res[total_len - 1 - i] = left_bytes[i];
        }

        if total_len % 2 != 0 {
            res[partition] = s_bytes[partition];
        }

        String::from_utf8(res).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3518() {
        assert_eq!(
            Solution::smallest_palindrome("abba".to_string(), 2),
            "baab".to_string()
        );
        assert_eq!(
            Solution::smallest_palindrome("aa".to_string(), 2),
            "".to_string()
        );
        assert_eq!(
            Solution::smallest_palindrome("bacab".to_string(), 1),
            "abcba".to_string()
        );
    }
}

fn main() {}
