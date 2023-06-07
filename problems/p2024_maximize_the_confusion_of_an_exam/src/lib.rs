pub struct Solution {}

impl Solution {
    pub fn is_ok(count: usize, answer_key: &str, k: usize) -> bool {
        let mut counts = [0; 2];
        for i in 0..answer_key.len() {
            counts[if answer_key.as_bytes()[i] == b'T' {
                0
            } else {
                1
            }] += 1;
            if count <= i {
                counts[if answer_key.as_bytes()[i - count] == b'T' {
                    0
                } else {
                    1
                }] -= 1;
            }
            if count - 1 <= i && counts[0].min(counts[1]) <= k {
                return true;
            }
        }

        false
    }

    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut ok = 1;
        let mut ng = answer_key.len() + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, &answer_key, k as usize) {
                ok = mid;
            } else {
                ng = mid
            }
        }

        ok as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2024() {
        assert_eq!(Solution::max_consecutive_answers("TTTF".to_string(), 2), 4);
        assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
        assert_eq!(
            Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1),
            5
        );
    }
}
