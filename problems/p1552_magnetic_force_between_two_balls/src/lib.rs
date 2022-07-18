pub struct Solution {}

impl Solution {
    pub fn is_ok(dist: i32, positon: &Vec<i32>, m: i32) -> bool {
        let mut prev = positon[0];
        let mut remains = m - 1;

        for i in 1..positon.len() {
            if dist <= positon[i] - prev {
                remains -= 1;
                if remains == 0 {
                    break;
                }
                prev = positon[i];
            }
        }

        remains == 0
    }

    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        let mut ok = 1;
        let mut ng = *position.last().unwrap();

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &position, m) {
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
    fn test_1552() {
        assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
        assert_eq!(
            Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
            999999999
        );
    }
}
