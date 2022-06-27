pub struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut remains = num_bottles;
        let mut result = num_bottles;

        while num_exchange <= remains {
            result += remains / num_exchange;
            remains = remains % num_exchange + remains / num_exchange;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1518() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
    }
}
