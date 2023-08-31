pub struct Solution {}

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut ranges = (0..ranges.len())
            .filter(|i| 0 < ranges[*i])
            .map(|i| (i as i32 - ranges[i], i as i32 + ranges[i]))
            .collect::<Vec<_>>();
        ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

        let mut j = 0;
        let mut right = -1;
        let mut count = 0;
        for i in 0..=n {
            if i < right || (i == n && i == right) {
                continue;
            }
            while j < ranges.len() && ranges[j].0 <= i {
                right = right.max(ranges[j].1);
                j += 1;
            }
            if right <= i {
                return -1;
            }
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1236() {
        assert_eq!(Solution::min_taps(3, vec![1, 0, 0, 1]), -1);
        assert_eq!(
            Solution::min_taps(
                35,
                vec![
                    1, 0, 4, 0, 4, 1, 4, 3, 1, 1, 1, 2, 1, 4, 0, 3, 0, 3, 0, 3, 0, 5, 3, 0, 0, 1,
                    2, 1, 2, 4, 3, 0, 1, 0, 5, 2
                ]
            ),
            6
        );
        assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
        assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
    }
}
