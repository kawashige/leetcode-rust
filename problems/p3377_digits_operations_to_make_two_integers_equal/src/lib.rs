use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

impl Solution {
    pub fn min_operations(n: i32, m: i32) -> i32 {
        let mut is_prime = vec![true; 10usize.pow(n.to_string().len() as u32 + 1)];
        is_prime[0] = false;
        is_prime[1] = false;
        for i in 2..is_prime.len() {
            if i * i > is_prime.len() {
                break;
            }
            if !is_prime[i] {
                continue;
            }
            for j in (i + i..is_prime.len()).step_by(i) {
                is_prime[j] = false;
            }
        }
        if is_prime[n as usize] || is_prime[m as usize] {
            return -1;
        }

        let mut heap = BinaryHeap::new();
        let mut seen = vec![std::i32::MAX; is_prime.len()];
        heap.push((Reverse(n), n));
        while let Some((Reverse(cost), val)) = heap.pop() {
            if seen[val as usize] <= cost {
                continue;
            }
            seen[val as usize] = cost;
            if val == m {
                return cost;
            }
            let chars = val.to_string().chars().collect::<Vec<_>>();
            for j in 0..chars.len() {
                if chars[j] != '0' {
                    let mut new_chars = chars.clone();
                    new_chars[j] = (chars[j] as u8 - 1) as char;
                    let d = new_chars
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    if !is_prime[d as usize] {
                        heap.push((Reverse(cost + d), d));
                    }
                }
                if chars[j] != '9' {
                    let mut new_chars = chars.clone();
                    new_chars[j] = (chars[j] as u8 + 1) as char;
                    let d = new_chars
                        .into_iter()
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                    if !is_prime[d as usize] {
                        heap.push((Reverse(cost + d), d));
                    }
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3377() {
        assert_eq!(Solution::min_operations(10, 12), 85);
        assert_eq!(Solution::min_operations(4, 8), -1);
        assert_eq!(Solution::min_operations(6, 2), -1);
    }
}
