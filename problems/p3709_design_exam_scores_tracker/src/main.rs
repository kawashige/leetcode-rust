struct ExamTracker {
    acc_scores: Vec<(i32, i64)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamTracker {
    fn new() -> Self {
        Self {
            acc_scores: vec![(0, 0)],
        }
    }

    fn record(&mut self, time: i32, score: i32) {
        self.acc_scores
            .push((time, self.acc_scores.last().unwrap().1 + score as i64));
    }

    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let s = match self
            .acc_scores
            .binary_search_by_key(&start_time, |(t, s)| *t)
        {
            Ok(i) => i - 1,
            Err(i) => i - 1,
        };
        let e = match self.acc_scores.binary_search_by_key(&end_time, |(t, s)| *t) {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        self.acc_scores[e].1 - self.acc_scores[s].1
    }
}

/**
 * Your ExamTracker object will be instantiated and called as such:
 * let obj = ExamTracker::new();
 * obj.record(time, score);
 * let ret_2: i64 = obj.total_score(startTime, endTime);
 */

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3709() {
        let mut obj = ExamTracker::new();
        obj.record(1, 98);
        assert_eq!(obj.total_score(1, 1), 98);
        obj.record(5, 99);
        assert_eq!(obj.total_score(1, 3), 98);
        assert_eq!(obj.total_score(1, 5), 197);
        assert_eq!(obj.total_score(3, 4), 0);
        assert_eq!(obj.total_score(2, 5), 99);
    }
}

fn main() {}
