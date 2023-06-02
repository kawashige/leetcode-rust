pub struct Solution {}

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;

        for i in 0..bombs.len() {
            let mut seen = vec![false; bombs.len()];
            let mut stack = vec![i];
            let mut count = 0;
            while let Some(j) = stack.pop() {
                if seen[j] {
                    continue;
                }
                seen[j] = true;
                count += 1;

                for k in 0..bombs.len() {
                    if seen[k] {
                        continue;
                    }
                    let d = ((bombs[j][0] - bombs[k][0]) as i128).pow(2)
                        + ((bombs[j][1] - bombs[k][1]) as i128).pow(2);
                    if d <= (bombs[j][2] as i128).pow(2) {
                        stack.push(k);
                    }
                }
            }
            max = max.max(count);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2101() {
        assert_eq!(
            Solution::maximum_detonation(vec![
                vec![54, 95, 4],
                vec![99, 46, 3],
                vec![29, 21, 3],
                vec![96, 72, 8],
                vec![49, 43, 3],
                vec![11, 20, 3],
                vec![2, 57, 1],
                vec![69, 51, 7],
                vec![97, 1, 10],
                vec![85, 45, 2],
                vec![38, 47, 1],
                vec![83, 75, 3],
                vec![65, 59, 3],
                vec![33, 4, 1],
                vec![32, 10, 2],
                vec![20, 97, 8],
                vec![35, 37, 3]
            ]),
            1
        );
        assert_eq!(
            Solution::maximum_detonation(vec![vec![1, 1, 100000], vec![100000, 100000, 1]]),
            1
        );
        assert_eq!(
            Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]),
            2
        );
        assert_eq!(
            Solution::maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]]),
            1
        );
        assert_eq!(
            Solution::maximum_detonation(vec![
                vec![1, 2, 3],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 3],
                vec![5, 6, 4]
            ]),
            5
        );
    }
}
