pub struct Solution {}

impl Solution {
    pub fn recurse(
        i: usize,
        topping_costs: &[i32],
        cost: i32,
        target: i32,
        closest_cost: &mut i32,
    ) {
        if topping_costs.len() == i {
            let diff = (target - cost).abs();
            let closest_diff = (target - *closest_cost).abs();
            if (closest_diff == diff && &cost < closest_cost) || diff < closest_diff {
                *closest_cost = cost;
            }
            return;
        }

        for j in 0..3 {
            Self::recurse(
                i + 1,
                topping_costs,
                cost + topping_costs[i] * j,
                target,
                closest_cost,
            );
        }
    }

    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut closest_cost = std::i32::MAX;

        for i in 0..base_costs.len() {
            Self::recurse(0, &topping_costs, base_costs[i], target, &mut closest_cost);
        }

        closest_cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1774() {
        assert_eq!(Solution::closest_cost(vec![1, 7], vec![3, 4], 10), 10);
        assert_eq!(Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18), 17);
        assert_eq!(Solution::closest_cost(vec![3, 10], vec![2, 5], 9), 8);
    }
}
