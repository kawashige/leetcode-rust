use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut list = vec![vec![]; n as usize];
        for d in dislikes {
            list[d[0] as usize - 1].push(d[1] as usize - 1);
            list[d[1] as usize - 1].push(d[0] as usize - 1);
        }

        let mut color = vec![-1; n as usize];
        for i in 0..(n as usize) {
            if color[i] != -1 {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((i, 0));

            while let Some((v, c)) = queue.pop_front() {
                color[v] = c;

                for next in &list[v] {
                    if color[*next] == (c + 1) % 2 {
                        continue;
                    } else if color[*next] == c {
                        return false;
                    }
                    queue.push_back((*next, (c + 1) % 2));
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0886() {
        assert!(!Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![3, 4], vec![4, 5], vec![3, 5]]
        ));
        assert!(Solution::possible_bipartition(
            4,
            vec![vec![1, 2], vec![1, 3], vec![2, 4]]
        ));
        assert!(!Solution::possible_bipartition(
            3,
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        ));
        assert!(!Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
        ));
    }
}
