pub struct Solution {}

impl Solution {
    pub fn recurse(
        city: usize,
        prev_city: usize,
        list: &Vec<Vec<usize>>,
        seats: i64,
    ) -> (i64, i64) {
        let mut cost = 0;
        let mut cities = 1;

        for next in &list[city] {
            if next == &prev_city {
                continue;
            }
            let (next_cost, next_cities) = Self::recurse(*next, city, list, seats);
            cost += next_cost;
            cities += next_cities;
        }

        (cost + (cities + seats - 1) / seats, cities)
    }

    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut list = vec![vec![]; roads.len() + 1];
        for road in roads {
            list[road[0] as usize].push(road[1] as usize);
            list[road[1] as usize].push(road[0] as usize);
        }

        list[0]
            .iter()
            .map(|i| Self::recurse(*i, 0, &list, seats as i64).0)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2477() {
        assert_eq!(
            Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5),
            3
        );
        assert_eq!(
            Solution::minimum_fuel_cost(
                vec![
                    vec![3, 1],
                    vec![3, 2],
                    vec![1, 0],
                    vec![0, 4],
                    vec![0, 5],
                    vec![4, 6]
                ],
                2
            ),
            7
        );
        assert_eq!(Solution::minimum_fuel_cost(vec![], 1), 0);
    }
}
