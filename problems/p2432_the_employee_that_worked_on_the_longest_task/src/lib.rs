pub struct Solution {}

impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut id = n;

        for i in 0..logs.len() {
            let time = logs[i][1] - if i == 0 { 0 } else { logs[i - 1][1] };
            if max < time {
                max = time;
                id = logs[i][0];
            } else if max == time {
                id = id.min(logs[i][0]);
            }
        }

        id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2432() {
        assert_eq!(
            Solution::hardest_worker(10, vec![vec![0, 3], vec![2, 5], vec![0, 9], vec![1, 15]]),
            1
        );
        assert_eq!(
            Solution::hardest_worker(26, vec![vec![1, 1], vec![3, 7], vec![2, 12], vec![7, 17]]),
            3
        );
        assert_eq!(
            Solution::hardest_worker(2, vec![vec![0, 10], vec![1, 20]]),
            0
        );
    }
}
