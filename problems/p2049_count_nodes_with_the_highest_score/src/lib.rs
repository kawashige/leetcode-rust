pub struct Solution {}

impl Solution {
    pub fn count_children(i: usize, children: &Vec<Vec<usize>>, children_count: &mut Vec<usize>) {
        for j in &children[i] {
            Self::count_children(*j, children, children_count);
            children_count[i] += children_count[*j];
        }
    }

    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let mut children = vec![vec![]; parents.len()];
        let mut root = 0;
        for i in 0..parents.len() {
            if parents[i] == -1 {
                root = i;
            } else {
                children[parents[i] as usize].push(i);
            }
        }
        let mut children_count = vec![1; parents.len()];
        Self::count_children(root, &children, &mut children_count);

        let mut max = 0;
        let mut count = 0;
        for i in 0..parents.len() {
            let mut product = 1;
            if parents[i] != -1 {
                product *= parents.len() - children_count[i];
            }
            for j in &children[i] {
                product *= children_count[*j];
            }
            if max < product {
                max = product;
                count = 1;
            } else if max == product {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2049() {
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0, 2, 0]), 3);
        assert_eq!(Solution::count_highest_score_nodes(vec![-1, 2, 0]), 2);
    }
}
