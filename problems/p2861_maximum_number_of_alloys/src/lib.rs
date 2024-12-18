pub struct Solution {}

impl Solution {
    pub fn is_ok(
        alloys: i32,
        budget: i32,
        composition: &Vec<Vec<i32>>,
        stock: &[i32],
        cost: &[i32],
    ) -> bool {
        composition.iter().any(|c| {
            (0..c.len())
                .map(|i| (c[i] as i64 * alloys as i64 - stock[i] as i64).max(0) * cost[i] as i64)
                .sum::<i64>()
                <= budget as i64
        })
    }

    pub fn max_number_of_alloys(
        n: i32,
        k: i32,
        budget: i32,
        composition: Vec<Vec<i32>>,
        stock: Vec<i32>,
        cost: Vec<i32>,
    ) -> i32 {
        let mut ok = 0;
        let mut ng = std::i32::MAX;

        while (ok + 1) < ng {
            let mid = (ok + ng) / 2;
            if Self::is_ok(mid, budget, &composition, &stock, &cost) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2861() {
        assert_eq!(
            Solution::max_number_of_alloys(
                3,
                2,
                15,
                vec![vec![1, 1, 1], vec![1, 1, 10]],
                vec![0, 0, 0],
                vec![1, 2, 3]
            ),
            2
        );
        assert_eq!(
            Solution::max_number_of_alloys(
                3,
                2,
                15,
                vec![vec![1, 1, 1], vec![1, 1, 10]],
                vec![0, 0, 100],
                vec![1, 2, 3]
            ),
            5
        );
        assert_eq!(
            Solution::max_number_of_alloys(
                2,
                3,
                10,
                vec![vec![2, 1], vec![1, 2], vec![1, 1]],
                vec![1, 1],
                vec![5, 5]
            ),
            2
        );
    }
}
