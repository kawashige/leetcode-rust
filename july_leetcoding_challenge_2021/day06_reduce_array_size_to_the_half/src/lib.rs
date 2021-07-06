pub struct Solution {}

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let mut count = arr.iter().fold(vec![0; 100_000], |mut count, x| {
            count[*x as usize - 1] += 1;
            count
        });

        count.sort_unstable_by(|a, b| b.cmp(&a));

        let mut i = 0;
        let mut sum = 0;

        while sum < arr.len() / 2 {
            sum += count[i];
            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06() {
        assert_eq!(
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]),
            2
        );
        assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(Solution::min_set_size(vec![1, 9]), 1);
        assert_eq!(Solution::min_set_size(vec![1000, 1000, 3, 7]), 1);
        assert_eq!(
            Solution::min_set_size(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
            5
        );
    }
}
