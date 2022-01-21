pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let diff_acc = gas
            .iter()
            .zip(cost.iter())
            .map(|(g, c)| g - c)
            .scan(0, |acc, d| {
                *acc += d;
                Some(*acc)
            })
            .collect::<Vec<_>>();

        if diff_acc.last().unwrap() < &0 {
            -1
        } else {
            (((0..diff_acc.len()).min_by_key(|i| diff_acc[*i]).unwrap() + 1) % diff_acc.len())
                as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0134() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 0, 1, 2, 3, 4, 0], vec![0, 1, 0, 0, 0, 0, 11]),
            0
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
