pub struct Solution {}

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; n as usize];
        for i in 0..edges.len() {
            count[edges[i][1] as usize] += 1;
        }
        if 1 < count.iter().filter(|c| c == &&0).count() {
            -1
        } else {
            (0..count.len()).find(|i| count[*i] == 0).unwrap() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2924() {
        assert_eq!(Solution::find_champion(3, vec![vec![0, 1], vec![1, 2]]), 0);
        assert_eq!(
            Solution::find_champion(4, vec![vec![0, 2], vec![1, 3], vec![1, 2]]),
            -1
        );
    }
}
