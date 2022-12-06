pub struct Solution {}

impl Solution {
    pub fn sum(n: usize, max: usize) -> usize {
        if n == max {
            max * (max + 1) / 2
        } else if n < max {
            max * (max + 1) / 2 - (max - n) * (max - n + 1) / 2
        } else {
            max * (max + 1) / 2 + n - max
        }
    }

    pub fn is_ok(n: usize, index: usize, max_sum: usize, mid: usize) -> bool {
        mid + Self::sum(index, mid - 1) + Self::sum(n - index - 1, mid - 1) <= max_sum
    }

    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let mut ok = 1;
        let mut ng = max_sum as usize + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(n as usize, index as usize, max_sum as usize, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1802() {
        assert_eq!(Solution::max_value(4, 2, 6), 2);
        assert_eq!(Solution::max_value(6, 1, 10), 3);
    }
}
