pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let l = gas.len();
        for i in 0..l {
            if gas[i] < cost[i] {
                continue;
            }
            let mut j = i;
            let mut remains = 0;
            loop {
                remains += gas[j] - cost[j];
                if remains < 0 {
                    break;
                }
                j = (j + 1) % l;
                if i == j {
                    return i as i32;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0134() {
        assert_eq!(
            3,
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
        );
        assert_eq!(
            -1,
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3])
        );
        assert_eq!(0, Solution::can_complete_circuit(vec![5], vec![4]));
    }
}
