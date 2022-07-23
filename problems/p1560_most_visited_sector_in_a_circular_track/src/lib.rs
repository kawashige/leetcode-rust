pub struct Solution {}

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; n as usize + 1];
        for i in 1..rounds.len() {
            if rounds[i - 1] < rounds[i] {
                for j in rounds[i - 1]..rounds[i] {
                    count[j as usize] += 1;
                }
            } else {
                for j in rounds[i - 1]..=n {
                    count[j as usize] += 1;
                }
                for j in 1..rounds[i] {
                    count[j as usize] += 1;
                }
            }
        }
        count[*rounds.last().unwrap() as usize] += 1;
        let max = count.iter().max().unwrap();
        (1..=n).filter(|i| &count[*i as usize] == max).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1560() {
        assert_eq!(Solution::most_visited(4, vec![1, 3, 1, 2]), vec![1, 2]);
        assert_eq!(
            Solution::most_visited(2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::most_visited(7, vec![1, 3, 5, 7]),
            vec![1, 2, 3, 4, 5, 6, 7]
        );
    }
}
