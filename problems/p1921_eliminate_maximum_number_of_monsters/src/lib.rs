pub struct Solution {}

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time = dist
            .iter()
            .zip(speed.iter())
            .map(|(d, s)| d / s - if d % s == 0 { 1 } else { 0 })
            .collect::<Vec<_>>();
        time.sort_unstable();

        for i in 1..time.len() {
            if time[i] < i as i32 {
                return i as i32;
            }
        }

        time.len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1921() {
        assert_eq!(
            Solution::eliminate_maximum(vec![3, 5, 7, 4, 5], vec![2, 3, 6, 3, 2]),
            2
        );
        assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
        assert_eq!(
            Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
            1
        );
        assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
    }
}
