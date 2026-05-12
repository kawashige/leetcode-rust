pub struct Solution {}

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut energy = tasks.iter().map(|t| t[0]).sum::<i32>();
        let mut tasks = tasks;
        tasks.sort_unstable_by_key(|t| -(t[1] - t[0]));

        let mut result = energy;

        for t in tasks {
            if energy < t[1] {
                result += t[1] - energy;
                energy = t[1]
            }
            energy -= t[0];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1665() {
        assert_eq!(
            Solution::minimum_effort(vec![vec![1, 2], vec![2, 4], vec![4, 8]]),
            8
        );
        assert_eq!(
            Solution::minimum_effort(vec![
                vec![1, 3],
                vec![2, 4],
                vec![10, 11],
                vec![10, 12],
                vec![8, 9]
            ]),
            32
        );
        assert_eq!(
            Solution::minimum_effort(vec![
                vec![1, 7],
                vec![2, 8],
                vec![3, 9],
                vec![4, 10],
                vec![5, 11],
                vec![6, 12]
            ]),
            27
        );
    }
}

fn main() {}
