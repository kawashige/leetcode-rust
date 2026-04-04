pub struct Solution {}

impl Solution {
    pub fn count_substrings(l: usize) -> usize {
        ((l + 1) * l) / 2
    }

    pub fn is_ok(mid: i32, time: &[i32], l: usize, k: usize) -> bool {
        let mut replaced = vec![-1];
        for i in 0..time.len() {
            if time[i] <= mid {
                replaced.push(i as i32);
            }
        }
        replaced.push(l as i32);

        let mut count = 0;
        for i in 1..replaced.len() {
            count += Self::count_substrings((replaced[i] - replaced[i - 1] - 1) as usize);
        }
        k + count <= Self::count_substrings(l)
    }

    pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut time = vec![0; s.len()];
        for i in 0..order.len() {
            time[order[i] as usize] = i as i32 + 1;
        }

        if Self::count_substrings(s.len()) < k {
            return -1;
        }

        let mut ng = 0;
        let mut ok = s.len() as i32;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &time, s.len(), k) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3639() {
        assert_eq!(Solution::min_time("abc".to_string(), vec![1, 0, 2], 2), 0);
        assert_eq!(Solution::min_time("cat".to_string(), vec![0, 2, 1], 6), 2);
        assert_eq!(Solution::min_time("xy".to_string(), vec![0, 1], 4), -1);
    }
}
