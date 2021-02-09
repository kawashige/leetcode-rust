pub struct Solution {}

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        fn dfs(
            price: &Vec<i32>,
            special: &Vec<Vec<i32>>,
            remains: &mut Vec<i32>,
            current_cost: i32,
            min_cost: &mut i32,
        ) {
            if remains.iter().all(|r| r == &0) {
                *min_cost = std::cmp::min(*min_cost, current_cost);
            }
            for i in 0..special.len() {
                if (0..remains.len()).any(|j| remains[j] < special[i][j]) {
                    continue;
                }
                for j in 0..remains.len() {
                    remains[j] -= special[i][j];
                }
                dfs(
                    price,
                    special,
                    remains,
                    current_cost + special[i].last().unwrap(),
                    min_cost,
                );
                for j in 0..remains.len() {
                    remains[j] += special[i][j];
                }
            }
            let without_special: i32 = (0..price.len()).map(|i| price[i] * remains[i]).sum();
            *min_cost = std::cmp::min(*min_cost, current_cost + without_special);
        }

        let mut min_cost = std::i32::MAX;
        let mut remains = needs;
        dfs(&price, &special, &mut remains, 0, &mut min_cost);
        min_cost
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0638() {
        assert_eq!(
            Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2]),
            14
        );
        assert_eq!(
            Solution::shopping_offers(
                vec![2, 3, 4],
                vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
                vec![1, 2, 1]
            ),
            11
        );
    }
}
