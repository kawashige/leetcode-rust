pub struct Solution {}

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let max_attack = properties[properties.len() - 1][0];
        let mut max_defence = 0;
        let mut cur_max_defence = properties[properties.len() - 1][1];

        let mut count = 0;

        for i in (0..properties.len() - 1).rev() {
            if properties[i][0] != properties[i + 1][0] {
                max_defence = max_defence.max(cur_max_defence);
            }
            if properties[i][0] < max_attack && properties[i][1] < max_defence {
                count += 1;
            }
            cur_max_defence = cur_max_defence.max(properties[i][1]);
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1996() {
        assert_eq!(
            Solution::number_of_weak_characters(vec![
                vec![1, 1],
                vec![2, 1],
                vec![2, 2],
                vec![1, 2]
            ]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
            0
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![2, 2], vec![3, 3]]),
            1
        );
        assert_eq!(
            Solution::number_of_weak_characters(vec![vec![1, 5], vec![10, 4], vec![4, 3]]),
            1
        );
    }
}
