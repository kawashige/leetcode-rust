pub struct Solution {}

impl Solution {
    pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
        tasks.into_iter().map(|t| t[0] + t[1]).min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3683() {
        assert_eq!(Solution::earliest_time(vec![vec![1, 6], vec![2, 3]]), 5);
        assert_eq!(
            Solution::earliest_time(vec![vec![100, 100], vec![100, 100], vec![100, 100]]),
            200
        );
    }
}

fn main() {}
