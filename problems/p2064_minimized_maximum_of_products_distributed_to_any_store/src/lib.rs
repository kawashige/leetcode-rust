pub struct Solution {}

impl Solution {
    pub fn is_ok(target: i32, n: i32, quantities: &[i32]) -> bool {
        quantities
            .iter()
            .map(|q| (q + target - 1) / target)
            .sum::<i32>()
            <= n
    }

    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut ng = 0;
        let mut ok = 100_001;

        while ng + 1 < ok {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, n, &quantities) {
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
    fn test_2064() {
        assert_eq!(Solution::minimized_maximum(6, vec![11, 6]), 3);
        assert_eq!(Solution::minimized_maximum(7, vec![15, 10, 10]), 5);
        assert_eq!(Solution::minimized_maximum(1, vec![100000]), 100000);
    }
}
