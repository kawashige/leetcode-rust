pub struct Solution {}

impl Solution {
    pub fn is_ok(mid: i64, start: &[i32], d: i64) -> bool {
        let mut val = start[0] as i64;
        for i in 1..start.len() {
            val = (val + mid).max(start[i] as i64);
            if !(start[i] as i64..=start[i] as i64 + d).contains(&val) {
                return false;
            }
        }
        true
    }

    pub fn max_possible_score(start: Vec<i32>, d: i32) -> i32 {
        let mut start = start;
        start.sort_unstable();

        if start.len() == 2 {
            return start[1] + d - start[0];
        }

        let mut ok = 0;
        let mut ng = (start[start.len() - 1] + d - start[0]) as i64;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &start, d as i64) {
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
    fn test_3281() {
        assert_eq!(
            Solution::max_possible_score(vec![100, 1000000000, 0], 1009),
            1109
        );
        assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
        assert_eq!(Solution::max_possible_score(vec![2, 6, 13, 13], 5), 5);
    }
}
