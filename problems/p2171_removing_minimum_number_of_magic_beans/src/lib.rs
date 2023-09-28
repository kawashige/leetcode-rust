pub struct Solution {}

impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut count = vec![0; 100_001];
        for bean in &beans {
            count[*bean as usize] += 1;
        }

        let mut acc_count = vec![0; 100_001];
        let mut acc_sum = vec![0; 100_001];

        for i in 0..count.len() {
            acc_count[i] = count[i];
            acc_sum[i] = i as i64 * count[i];
            if 0 < i {
                acc_count[i] += acc_count[i - 1];
                acc_sum[i] += acc_sum[i - 1];
            }
        }

        let mut min = std::i64::MAX;

        for i in 0..count.len() {
            let removal = if i == 0 { 0 } else { acc_sum[i - 1] } + acc_sum[acc_sum.len() - 1]
                - acc_sum[i]
                - i as i64 * (acc_count[acc_count.len() - 1] - acc_count[i]);
            min = min.min(removal);
        }

        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2171() {
        assert_eq!(Solution::minimum_removal(vec![4, 1, 6, 5]), 4);
        assert_eq!(Solution::minimum_removal(vec![2, 10, 3, 2]), 7);
    }
}
