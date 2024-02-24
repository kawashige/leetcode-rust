use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut list = vec![vec![]; n as usize];
        list[0].push((first_person as usize, 0));
        for meeting in meetings {
            list[meeting[0] as usize].push((meeting[1] as usize, meeting[2]));
            list[meeting[1] as usize].push((meeting[0] as usize, meeting[2]));
        }
        let mut seen = vec![std::i32::MAX; n as usize];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((i, t)) = queue.pop_front() {
            if seen[i] <= t {
                continue;
            }
            seen[i] = t;
            for (next, time) in &list[i] {
                if time < &t {
                    continue;
                }
                queue.push_back((*next, *time));
            }
        }

        (0..n)
            .filter(|i| seen[*i as usize] != std::i32::MAX)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2092() {
        assert_eq!(
            Solution::find_all_people(6, vec![vec![0, 2, 1], vec![1, 3, 1], vec![4, 5, 1]], 1),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1),
            vec![0, 1, 2, 3, 5]
        );
        assert_eq!(
            Solution::find_all_people(4, vec![vec![3, 1, 3], vec![1, 2, 2], vec![0, 3, 3]], 3),
            vec![0, 1, 3]
        );
        assert_eq!(
            Solution::find_all_people(5, vec![vec![3, 4, 2], vec![1, 2, 1], vec![2, 3, 1]], 1),
            vec![0, 1, 2, 3, 4]
        );
    }
}
