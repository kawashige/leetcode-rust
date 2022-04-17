pub struct Solution {}

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut exist = vec![false; 2001];
        for x in arr2 {
            exist[(x + 1000) as usize] = true;
        }
        arr1.into_iter()
            .filter(|x| {
                ((x - d).max(-1000)..=(x + d).min(1000)).all(|x| !exist[(x + 1000) as usize])
            })
            .count() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1385() {
        assert_eq!(
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
            2
        );
        assert_eq!(
            Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
            1
        );
    }
}
