pub struct Solution {}

impl Solution {
    pub fn sort_items(n: i32, m: i32, group: Vec<i32>, before_items: Vec<Vec<i32>>) -> Vec<i32> {
        let mut group_items = vec![vec![]; m as usize];
        for i in 0..group.len() {
            if group[i] != -1 {
                group_items[group[i] as usize].push(i);
            }
        }

        let mut ingroup_before_count = vec![0; n as usize];
        let mut ingroup_after_items = vec![vec![]; n as usize];
        for i in 0..before_items.len() {
            for j in &before_items[i] {
                if group[i] == group[*j as usize] {
                    ingroup_after_items[*j as usize].push(i);
                    ingroup_before_count[i] += 1;
                }
            }
        }

        let mut sorted_group_item = vec![vec![]; m as usize];
        let mut sorted = vec![false; n as usize];
        for i in 0..m as usize {
            while sorted_group_item[i].len() < group_items[i].len() {
                let mut found = false;
                for j in &group_items[i] {
                    if !sorted[*j] && ingroup_before_count[*j] == 0 {
                        sorted_group_item[i].push(*j);
                        sorted[*j] = true;
                        for k in &ingroup_after_items[*j] {
                            ingroup_before_count[*k] -= 1;
                        }
                        found = true;
                    }
                }
                if !found {
                    return Default::default();
                }
            }
        }

        let mut before_count = vec![0; n as usize];
        let mut after_items = vec![vec![]; n as usize];
        for i in 0..before_items.len() {
            for j in &before_items[i] {
                if group[i] == -1 || group[*j as usize] == -1 || group[i] != group[*j as usize] {
                    after_items[if group[*j as usize] == -1 {
                        *j as usize
                    } else {
                        sorted_group_item[group[*j as usize] as usize][0]
                    }]
                    .push(if group[i] == -1 {
                        i
                    } else {
                        sorted_group_item[group[i] as usize][0]
                    });
                    before_count[if group[i] == -1 {
                        i
                    } else {
                        sorted_group_item[group[i] as usize][0]
                    }] += 1;
                }
            }
        }

        let mut sorted_item = Vec::with_capacity(n as usize);
        let mut sorted = vec![false; n as usize];
        for i in 0..n as usize {
            if group[i] != -1 && sorted_group_item[group[i] as usize][0] != i {
                sorted[i] = true;
            }
        }
        while sorted_item.len() < n as usize {
            let mut found = false;
            for i in 0..n as usize {
                if !sorted[i] && before_count[i] == 0 {
                    sorted[i] = true;
                    for j in &after_items[i] {
                        before_count[*j] -= 1;
                    }
                    if group[i] == -1 {
                        sorted_item.push(i as i32);
                    } else {
                        for j in &sorted_group_item[group[i] as usize] {
                            sorted_item.push(*j as i32);
                        }
                    }
                    found = true;
                }
            }
            if !found {
                return Default::default();
            }
        }
        sorted_item
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1203() {
        assert_eq!(
            Solution::sort_items(
                5,
                5,
                vec![2, 0, -1, 3, 0],
                vec![vec![2, 1, 3], vec![2, 4], vec![], vec![], vec![]]
            ),
            vec![3, 2, 4, 1, 0]
        );
        assert_eq!(
            Solution::sort_items(
                8,
                2,
                vec![-1, -1, 1, 0, 0, 1, 0, -1],
                vec![
                    vec![],
                    vec![6],
                    vec![5],
                    vec![6],
                    vec![3, 6],
                    vec![],
                    vec![],
                    vec![]
                ]
            ),
            vec![0, 5, 2, 6, 3, 4, 7, 1]
        );
        assert_eq!(
            Solution::sort_items(
                8,
                2,
                vec![-1, -1, 1, 0, 0, 1, 0, -1],
                vec![
                    vec![],
                    vec![6],
                    vec![5],
                    vec![6],
                    vec![3],
                    vec![],
                    vec![4],
                    vec![]
                ]
            ),
            vec![]
        );
    }
}
