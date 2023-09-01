pub struct Solution {}

impl Solution {
    pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
        let mut left = vec![0; arr.len()];
        let mut count = vec![0; 100_001];
        let mut prev = vec![arr.len(); 100_001];

        for i in 0..arr.len() {
            if prev[arr[i] as usize] < arr.len() {
                left[i] += ((i - prev[arr[i] as usize]) * count[arr[i] as usize]) as i64
                    + left[prev[arr[i] as usize]];
            }
            prev[arr[i] as usize] = i;
            count[arr[i] as usize] += 1;
        }

        let mut right = vec![0; arr.len()];
        let mut count = vec![0; 100_001];
        let mut prev = vec![arr.len(); 100_001];

        for i in (0..arr.len()).rev() {
            if prev[arr[i] as usize] < arr.len() {
                right[i] += ((prev[arr[i] as usize] - i) * count[arr[i] as usize]) as i64
                    + right[prev[arr[i] as usize]];
            }
            prev[arr[i] as usize] = i;
            count[arr[i] as usize] += 1;
        }

        left.into_iter()
            .zip(right.into_iter())
            .map(|(l, r)| l + r)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2121() {
        assert_eq!(
            Solution::get_distances(vec![2, 1, 3, 1, 2, 3, 3]),
            vec![4, 2, 7, 2, 4, 4, 5]
        );
        assert_eq!(
            Solution::get_distances(vec![10, 5, 10, 10]),
            vec![5, 0, 3, 4]
        );
    }
}
