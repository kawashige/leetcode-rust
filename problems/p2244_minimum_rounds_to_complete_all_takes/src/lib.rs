pub struct Solution {}

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_unstable();

        let mut rounds = 0;
        let mut count = 0;
        for i in 0..tasks.len() {
            count += 1;
            if i == tasks.len() - 1 || tasks[i] != tasks[i + 1] {
                let mut round = None;
                for three in (0..=count / 3).rev() {
                    if (count - three * 3) % 2 == 0 {
                        round = Some(three + (count - three * 3) / 2);
                        break;
                    }
                }
                if let Some(round) = round {
                    rounds += round;
                } else {
                    return -1;
                }
                count = 0;
            }
        }

        rounds
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2244() {
        assert_eq!(
            Solution::minimum_rounds(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            4
        );
        assert_eq!(
            Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]),
            4
        );
        assert_eq!(Solution::minimum_rounds(vec![2, 3, 3]), -1);
    }
}
