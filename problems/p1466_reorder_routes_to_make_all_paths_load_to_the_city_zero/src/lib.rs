pub struct Solution {}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; n as usize];

        for connection in connections {
            list[connection[0] as usize].push((connection[1] as usize, 1));
            list[connection[1] as usize].push((connection[0] as usize, 0));
        }

        let mut stack = vec![0];
        let mut seen = vec![false; n as usize];
        let mut cost = 0;

        while let Some(i) = stack.pop() {
            if seen[i] {
                continue;
            }
            seen[i] = true;
            for (j, c) in &list[i] {
                if seen[*j] {
                    continue;
                }
                cost += c;
                stack.push(*j);
            }
        }

        cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1466() {
        assert_eq!(
            Solution::min_reorder(
                6,
                vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]],
            ),
            3
        );
        assert_eq!(
            Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]],),
            2
        );
        assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
    }
}
