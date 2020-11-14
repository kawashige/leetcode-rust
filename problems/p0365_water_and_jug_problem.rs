pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z == 0 || x == z || y == z || x + y == z || (x - y).abs() == z {
            return true;
        } else if x + y < z || x == y || x == 0 || y == 0 {
            return false;
        }

        let mut small = x;
        let mut large = y;
        if large < small {
            std::mem::swap(&mut small, &mut large);
        }

        if z % small == 0 && z / small == large / small || small == large / small + 1 {
            return true;
        }

        let mut rest = small - (large - (small * (large / small)));
        let mut opened = HashSet::new();
        while 0 < rest && !opened.contains(&rest) {
            opened.insert(rest);

            let mut current = rest;
            while current <= large {
                println!("rest: {}, current: {}", rest, current);
                if current == z {
                    return true;
                }
                current += small;
            }
            if current == z {
                return true;
            }
            rest = small - (large - (current - small))
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0365() {
        assert!(Solution::can_measure_water(11, 3, 0));
        assert!(Solution::can_measure_water(11, 3, 13));
        assert!(!Solution::can_measure_water(0, 2, 1));
        assert!(Solution::can_measure_water(0, 2, 2));
        assert!(Solution::can_measure_water(2, 5, 1));
        assert!(Solution::can_measure_water(3, 5, 4));
        assert!(!Solution::can_measure_water(2, 6, 5));
    }
}
