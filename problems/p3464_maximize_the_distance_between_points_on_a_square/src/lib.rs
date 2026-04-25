pub struct Solution {}

impl Solution {
    pub fn is_ok(min_dist: i64, k: i64, total_len: i64, coodinates: &[i64]) -> bool {
        for p in coodinates {
            let max = p + total_len - min_dist;
            let mut cur = *p;
            for _ in 0..k - 1 {
                let mut left = 0;
                let mut right = coodinates.len();
                while left < right {
                    let mid = (left + right) / 2;
                    if coodinates[mid] < cur + min_dist {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                if left == coodinates.len() || max < coodinates[left] {
                    cur = -1;
                    break;
                }
                cur = coodinates[left]
            }
            if 0 <= cur {
                return true;
            }
        }
        false
    }

    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut coodinates = Vec::with_capacity(points.len());

        for p in points {
            if p[0] == 0 {
                coodinates.push(p[1] as i64)
            } else if p[1] == side {
                coodinates.push(side as i64 + p[0] as i64)
            } else if p[0] == side {
                coodinates.push(side as i64 * 3 - p[1] as i64)
            } else {
                coodinates.push(side as i64 * 4 - p[0] as i64)
            }
        }

        coodinates.sort_unstable();

        let total_len = side as i64 * 4;
        let mut ok = 1;
        let mut ng = side as i64 + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, k as i64, total_len, &coodinates) {
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
    fn test_3464() {
        assert_eq!(
            Solution::max_distance(
                13,
                vec![
                    vec![5, 0],
                    vec![0, 3],
                    vec![9, 13],
                    vec![0, 0],
                    vec![0, 13],
                    vec![10, 13]
                ],
                4
            ),
            8
        );
        assert_eq!(
            Solution::max_distance(2, vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]], 4),
            2
        );
        assert_eq!(
            Solution::max_distance(
                2,
                vec![vec![0, 0], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]],
                4
            ),
            1
        );
        assert_eq!(
            Solution::max_distance(
                2,
                vec![
                    vec![0, 0],
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 2],
                    vec![2, 0],
                    vec![2, 2],
                    vec![2, 1]
                ],
                5
            ),
            1
        );
    }
}
