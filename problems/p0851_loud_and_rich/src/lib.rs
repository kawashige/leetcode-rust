pub struct Solution {}

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; quiet.len()];
        let mut parents = vec![vec![]; quiet.len()];
        let mut result: Vec<i32> = (0..(quiet.len() as i32)).collect();

        for rich in richer {
            count[rich[1] as usize] += 1;
            parents[rich[0] as usize].push(rich[1] as usize);
        }

        let mut stack = (0..count.len())
            .filter(|i| count[*i] == 0)
            .collect::<Vec<usize>>();

        while let Some(i) = stack.pop() {
            if parents[i].is_empty() {
                continue;
            }

            for parent in &parents[i] {
                if quiet[result[*parent] as usize] > quiet[result[i] as usize] {
                    result[*parent] = result[i];
                }
                count[*parent] -= 1;
                if count[*parent] == 0 {
                    stack.push(*parent);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0851() {
        assert_eq!(
            Solution::loud_and_rich(vec![vec![0, 1], vec![1, 2],], vec![1, 0, 2]),
            vec![0, 1, 1]
        );

        assert_eq!(
            Solution::loud_and_rich(
                vec![
                    vec![1, 0],
                    vec![2, 1],
                    vec![3, 1],
                    vec![3, 7],
                    vec![4, 3],
                    vec![5, 3],
                    vec![6, 3]
                ],
                vec![3, 2, 5, 4, 6, 1, 7, 0]
            ),
            vec![5, 5, 2, 5, 4, 5, 6, 7]
        );
    }
}
