use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct Solution {}

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let order_items = orders
            .iter()
            .map(|order| order[2].clone())
            .collect::<BTreeSet<_>>();

        let order_items_map = order_items
            .iter()
            .enumerate()
            .map(|(i, item)| (item, i))
            .collect::<HashMap<_, _>>();

        let table = orders.into_iter().fold(BTreeMap::new(), |mut map, order| {
            map.entry(order[1].parse::<usize>().unwrap())
                .or_insert(vec![0; order_items.len()])[order_items_map[&order[2]]] += 1;
            map
        });

        let mut result = Vec::with_capacity(table.len() + 1);
        result.push(
            std::iter::once("Table".to_string())
                .chain(order_items.into_iter())
                .collect(),
        );

        for (table, items) in table {
            result.push(
                std::iter::once(table.to_string())
                    .chain(items.iter().map(|count| count.to_string()))
                    .collect(),
            )
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1418() {
        assert_eq!(
            Solution::display_table(vec![
                vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
                vec![
                    "Corina".to_string(),
                    "10".to_string(),
                    "Beef Burrito".to_string()
                ],
                vec![
                    "David".to_string(),
                    "3".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
                vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
                vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Beef Burrito".to_string(),
                    "Ceviche".to_string(),
                    "Fried Chicken".to_string(),
                    "Water".to_string()
                ],
                vec![
                    "3".to_string(),
                    "0".to_string(),
                    "2".to_string(),
                    "1".to_string(),
                    "0".to_string()
                ],
                vec![
                    "5".to_string(),
                    "0".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                    "1".to_string()
                ],
                vec![
                    "10".to_string(),
                    "1".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                    "0".to_string()
                ]
            ]
        );
        assert_eq!(
            Solution::display_table(vec![
                vec![
                    "James".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Ratesh".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Amadeus".to_string(),
                    "12".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec![
                    "Adam".to_string(),
                    "1".to_string(),
                    "Canadian Waffles".to_string()
                ],
                vec![
                    "Brianna".to_string(),
                    "1".to_string(),
                    "Canadian Waffles".to_string()
                ]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Canadian Waffles".to_string(),
                    "Fried Chicken".to_string()
                ],
                vec!["1".to_string(), "2".to_string(), "0".to_string()],
                vec!["12".to_string(), "0".to_string(), "3".to_string()]
            ]
        );
        assert_eq!(
            Solution::display_table(vec![
                vec![
                    "Laura".to_string(),
                    "2".to_string(),
                    "Bean Burrito".to_string()
                ],
                vec![
                    "Jhon".to_string(),
                    "2".to_string(),
                    "Beef Burrito".to_string()
                ],
                vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()]
            ]),
            vec![
                vec![
                    "Table".to_string(),
                    "Bean Burrito".to_string(),
                    "Beef Burrito".to_string(),
                    "Soda".to_string()
                ],
                vec![
                    "2".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                    "1".to_string()
                ]
            ]
        );
    }
}
