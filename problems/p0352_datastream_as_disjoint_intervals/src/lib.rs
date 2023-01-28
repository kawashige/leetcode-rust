struct SummaryRanges {
    intervals: Vec<Vec<i32>>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self { intervals: vec![] }
    }

    fn add_num(&mut self, value: i32) {
        match self.intervals.binary_search_by_key(&value, |v| v[0]) {
            Ok(_) => {}
            Err(i) => {
                if 0 < i
                    && i < self.intervals.len()
                    && self.intervals[i - 1][1] + 1 == value
                    && value + 1 == self.intervals[i][0]
                {
                    self.intervals[i - 1][1] = self.intervals[i][1];
                    self.intervals.remove(i);
                } else if 0 < i && value <= self.intervals[i - 1][1] + 1 {
                    self.intervals[i - 1][1] = self.intervals[i - 1][1].max(value);
                } else if i < self.intervals.len() && value + 1 == self.intervals[i][0] {
                    self.intervals[i][0] -= 1;
                } else {
                    self.intervals.insert(i, vec![value, value])
                }
            }
        }
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.intervals.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0352() {
        let mut obj = SummaryRanges::new();
        obj.add_num(1);
        assert_eq!(obj.get_intervals(), vec![vec![1, 1]]);
        obj.add_num(3);
        assert_eq!(obj.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        obj.add_num(7);
        assert_eq!(
            obj.get_intervals(),
            vec![vec![1, 1], vec![3, 3], vec![7, 7]]
        );
        obj.add_num(2);
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
        obj.add_num(6);
        assert_eq!(obj.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}
