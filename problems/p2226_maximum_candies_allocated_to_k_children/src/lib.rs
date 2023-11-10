pub struct Solution {}

impl Solution {
    pub fn is_ok(candies: &[i32], k: i64, num: i64) -> bool {
        k <= candies.iter().map(|candie| *candie as i64 / num).sum()
    }

    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let sum: i64 = candies.iter().map(|candie| *candie as i64).sum();
        if sum < k {
            return 0;
        }

        let mut ok = 1;
        let mut ng = sum + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(&candies, k, mid) {
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
    fn test_2226() {
        assert_eq!(Solution::maximum_candies(vec![5, 8, 6], 3), 5);
        assert_eq!(Solution::maximum_candies(vec![2, 5], 11), 0);
    }
}
