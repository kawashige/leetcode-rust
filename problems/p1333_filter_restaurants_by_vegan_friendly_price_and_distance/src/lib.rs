pub struct Solution {}

impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut filtered = restaurants
            .into_iter()
            .filter(|r| {
                (vegan_friendly == 0 || r[2] == 1) && r[3] <= max_price && r[4] <= max_distance
            })
            .collect::<Vec<_>>();

        filtered.sort_unstable_by(|a, b| b[1].cmp(&a[1]).then(b[0].cmp(&a[0])));
        filtered.into_iter().map(|r| r[0]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1333() {
        assert_eq!(
            Solution::filter_restaurants(
                vec![
                    vec![1, 4, 1, 40, 10],
                    vec![2, 8, 0, 50, 5],
                    vec![3, 8, 1, 30, 4],
                    vec![4, 10, 0, 10, 3],
                    vec![5, 1, 1, 15, 1]
                ],
                1,
                50,
                10
            ),
            vec![3, 1, 5]
        );
        assert_eq!(
            Solution::filter_restaurants(
                vec![
                    vec![1, 4, 1, 40, 10],
                    vec![2, 8, 0, 50, 5],
                    vec![3, 8, 1, 30, 4],
                    vec![4, 10, 0, 10, 3],
                    vec![5, 1, 1, 15, 1]
                ],
                0,
                50,
                10
            ),
            vec![4, 3, 2, 1, 5]
        );
        assert_eq!(
            Solution::filter_restaurants(
                vec![
                    vec![1, 4, 1, 40, 10],
                    vec![2, 8, 0, 50, 5],
                    vec![3, 8, 1, 30, 4],
                    vec![4, 10, 0, 10, 3],
                    vec![5, 1, 1, 15, 1]
                ],
                0,
                30,
                3
            ),
            vec![4, 5]
        );
    }
}
