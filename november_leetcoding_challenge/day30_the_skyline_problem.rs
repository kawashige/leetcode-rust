pub struct Solution {}
use std::collections::BTreeMap;
impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut results = Vec::new();
        let mut heigths = buildings
            .into_iter()
            .map(|v| vec![vec![v[0], -v[2]], vec![v[1], v[2]]])
            .flatten()
            .collect::<Vec<Vec<i32>>>();
        heigths.sort_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut prev = 0;
        let mut current_heights: BTreeMap<i32, usize> = BTreeMap::new();
        println!("{:?}", heigths);
        for h in heigths {
            if h[1] < 0 {
                *current_heights.entry(-h[1]).or_default() += 1;
            } else {
                if *current_heights.get(&h[1]).unwrap() > 1 {
                    *current_heights.get_mut(&h[1]).unwrap() -= 1;
                } else {
                    current_heights.remove(&h[1]);
                }
            }

            let current = *current_heights.keys().last().unwrap_or(&0);
            if prev != current {
                results.push(vec![h[0], current]);
                prev = current;
            }
        }

        println!("{:?}", results);
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day30() {
        assert_eq!(
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ],
            Solution::get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ])
        );
        assert_eq!(
            vec![vec![2, 5], vec![12, 0]],
            Solution::get_skyline(vec![vec![2, 9, 5], vec![9, 12, 5]])
        );
        assert_eq!(
            vec![vec![2, 5], vec![3, 7], vec![5, 10], vec![9, 0]],
            Solution::get_skyline(vec![vec![2, 9, 5], vec![3, 9, 7], vec![5, 9, 10]])
        );
        assert_eq!(
            Vec::new() as Vec<Vec<i32>>,
            Solution::get_skyline(Vec::new())
        );
        assert_eq!(
            vec![vec![2, 10], vec![9, 15], vec![12, 0]],
            Solution::get_skyline(vec![vec![2, 9, 10], vec![9, 12, 15]])
        );
        assert_eq!(
            vec![vec![1, 3], vec![2, 0]],
            Solution::get_skyline(vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]])
        );
        assert_eq!(
            vec![vec![2, 10], vec![10, 25], vec![17, 14], vec![20, 0]],
            Solution::get_skyline(vec![vec![2, 13, 10], vec![10, 17, 25], vec![12, 20, 14]])
        );
    }
}
