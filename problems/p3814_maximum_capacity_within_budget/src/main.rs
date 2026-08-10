pub struct Solution {}

impl Solution {
    pub fn max_capacity(costs: Vec<i32>, capacity: Vec<i32>, budget: i32) -> i32 {
        let mut max_capacity = vec![0; capacity.len()];
        let mut machine = costs
            .into_iter()
            .zip(capacity.into_iter())
            .collect::<Vec<_>>();
        machine.sort_unstable();

        max_capacity[0] = machine[0].1;
        for i in 1..machine.len() {
            max_capacity[i] = machine[i].1.max(max_capacity[i - 1]);
        }

        let mut j = 0;
        let mut result = 0;
        for i in (0..machine.len()).rev() {
            if budget <= machine[i].0 {
                continue;
            }
            result = result.max(machine[i].1);
            if i <= j {
                if 0 < i {
                    j = i - 1;
                } else {
                    j = 0;
                }
            }
            while j + 1 < i && machine[i].0 + machine[j + 1].0 < budget {
                j += 1;
            }
            if i != j && machine[i].0 + machine[j].0 < budget {
                result = result.max(machine[i].1 + max_capacity[j]);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3814() {
        assert_eq!(
            Solution::max_capacity(vec![5, 5, 9, 5], vec![7, 9, 8, 8], 13),
            17
        );
        assert_eq!(
            Solution::max_capacity(vec![4, 8, 5, 3], vec![1, 5, 2, 7], 8),
            8
        );
        assert_eq!(
            Solution::max_capacity(vec![3, 5, 7, 4], vec![2, 4, 3, 6], 7),
            6
        );
        assert_eq!(Solution::max_capacity(vec![2, 2, 2], vec![3, 5, 4], 5), 9);
    }
}

fn main() {}
