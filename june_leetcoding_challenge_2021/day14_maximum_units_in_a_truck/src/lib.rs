pub struct Solution {}

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        box_types.sort_unstable_by(|a, b| b[1].cmp(&a[1]));

        let mut units = 0;
        for b in box_types {
            let c = std::cmp::min(b[0], truck_size);
            units += c * b[1];
            truck_size -= c;
            if truck_size == 0 {
                break;
            }
        }

        units
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day14() {
        assert_eq!(
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
        assert_eq!(
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
