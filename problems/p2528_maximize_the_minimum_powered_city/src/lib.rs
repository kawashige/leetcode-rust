use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, power: &[i64], k: i64, r: usize) -> bool {
        let mut queue = VecDeque::new();
        let mut plus = 0;
        let mut k = k;
        for i in 0..power.len() {
            while let Some((p, j)) = queue.pop_front() {
                if i <= j {
                    queue.push_front((p, j));
                    break;
                }
                plus -= p;
            }
            if mid <= power[i] + plus {
                continue;
            }
            if power[i] + plus + k < mid {
                return false;
            }
            let d = mid - power[i] - plus;
            queue.push_back((d, i + r * 2));
            k -= d;
            plus += d;
        }
        true
    }

    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;
        let mut power = vec![0; stations.len()];
        let mut p = 0;
        for i in 0..stations.len() {
            p += stations[i] as i64;
            if r < i {
                p -= stations[i - r - 1] as i64;
            }
            power[i] = p;
        }
        let mut p = 0;
        for i in (0..stations.len()).rev() {
            if i + 1 + r < stations.len() {
                p -= stations[i + r + 1] as i64;
            }
            power[i] += p;
            p += stations[i] as i64;
        }

        let mut ok = 0;
        let mut ng = std::i64::MAX;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &power, k, r) {
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
    fn test_2528() {
        assert_eq!(Solution::max_power(vec![1, 2, 4, 5, 0], 1, 2), 5);
        assert_eq!(Solution::max_power(vec![4, 4, 4, 4], 0, 3), 4);
    }
}
