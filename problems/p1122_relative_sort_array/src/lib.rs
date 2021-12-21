pub struct Solution {}

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let order = arr2
            .iter()
            .enumerate()
            .fold([arr2.len(); 1001], |mut index, (i, x)| {
                index[*x as usize] = i;
                index
            });

        arr1.sort_unstable_by(|a, b| order[*a as usize].cmp(&order[*b as usize]).then(a.cmp(b)));
        arr1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1122() {
        assert_eq!(
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
        assert_eq!(
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }
}
