pub struct Solution {}

impl Solution {
    pub fn is_ok(milestones: &[i32], mid: i64) -> bool {
        let max = (mid + 1) / 2;
        mid <= milestones.iter().map(|m| (*m as i64).min(max)).sum::<i64>()
    }

    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let mut ok = 1;
        let mut ng = milestones.iter().map(|m| *m as i64).sum::<i64>() + 1;

        while ok + 1 < ng {
            let mid = (ok + ng) / 2;
            println!("mid: {}", mid);
            if Self::is_ok(&milestones, mid) {
                println!("ok");
                ok = mid;
            } else {
                println!("ng");
                ng = mid
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1953() {
        assert_eq!(Solution::number_of_weeks(vec![1, 2, 3]), 6);
        assert_eq!(Solution::number_of_weeks(vec![5, 2, 1]), 7);
    }
}
