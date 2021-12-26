pub struct Solution {}

impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        let mut values = values
            .into_iter()
            .zip(labels.into_iter())
            .collect::<Vec<_>>();
        values.sort_unstable_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));

        let mut used_count = vec![0; 2 * 10_000 + 1];
        let mut remains = num_wanted;
        let mut result = 0;

        for i in 0..values.len() {
            if remains == 0 {
                break;
            }
            if used_count[values[i].1 as usize] == use_limit {
                continue;
            }
            remains -= 1;
            result += values[i].0;
            used_count[values[i].1 as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1090() {
        assert_eq!(
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
            9
        );
        assert_eq!(
            Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
            12
        );
        assert_eq!(
            Solution::largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1),
            16
        );
    }
}
