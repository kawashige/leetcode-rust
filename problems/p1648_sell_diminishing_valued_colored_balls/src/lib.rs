pub struct Solution {}

impl Solution {
    pub fn max_profit(inventory: Vec<i32>, orders: i32) -> i32 {
        const M: usize = 1_000_000_007;

        let mut inventory = std::iter::once((0, 0))
            .chain(inventory.into_iter().map(|num| (num as usize, 1)))
            .collect::<Vec<_>>();
        inventory.sort_unstable();

        let mut remains = orders as usize;
        let mut total_value = 0;

        while 0 < remains {
            let (i1, c1) = inventory.pop().unwrap();
            let (i2, _) = *inventory.last().unwrap();

            let diff = i1 - i2;
            if remains <= diff * c1 {
                let d1 = remains / c1;
                total_value = (total_value
                    + ((i1 * (i1 + 1)) / 2 - ((i1 - d1) * (i1 - d1 + 1)) / 2) * c1 % M)
                    % M;
                total_value = (total_value + (remains - d1 * c1) * (i1 - d1) % M) % M;
                remains = 0;
            } else {
                remains -= diff * c1;
                total_value =
                    (total_value + (i1 * (i1 + 1) / 2 - (i2 * (i2 + 1)) / 2) * c1 % M) % M;
            }
            let l = inventory.len();
            inventory[l - 1].1 += c1;
        }

        total_value as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1648() {
        assert_eq!(Solution::max_profit(vec![2, 8, 4, 10, 6], 20), 110);
        assert_eq!(Solution::max_profit(vec![2, 5], 4), 14);
        assert_eq!(Solution::max_profit(vec![3, 5], 6), 19);
    }
}
