pub struct Solution {}

impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut in_count = vec![0; n as usize];
        let mut next_courses = vec![vec![]; n as usize];

        for relation in relations {
            in_count[relation[1] as usize - 1] += 1;
            next_courses[relation[0] as usize - 1].push(relation[1] as usize - 1);
        }

        let mut months = vec![0; n as usize];
        let mut stack = (0..in_count.len())
            .filter(|i| in_count[*i] == 0)
            .collect::<Vec<_>>();

        while let Some(course) = stack.pop() {
            for next in &next_courses[course] {
                in_count[*next] -= 1;
                months[*next] = months[*next].max(months[course] + time[course]);
                if in_count[*next] == 0 {
                    stack.push(*next);
                }
            }
        }

        months
            .into_iter()
            .zip(time.into_iter())
            .map(|(m, t)| m + t)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2050() {
        assert_eq!(
            Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]),
            8
        );
        assert_eq!(
            Solution::minimum_time(
                5,
                vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
                vec![1, 2, 3, 4, 5]
            ),
            12
        );
    }
}
