pub struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const M: usize = 1_000_000_007;
        let mut ways = 1;
        let mut seats = 0;
        let mut plants = 0;

        for b in corridor.as_bytes() {
            if b == &b'S' {
                if seats == 2 {
                    ways = (ways * (plants + 1)) % M;
                    seats = 0;
                }
                seats += 1;
                if seats == 2 {
                    plants = 0;
                }
            } else {
                if seats == 2 {
                    plants += 1;
                }
            }
        }

        if seats == 2 {
            ways as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2147() {
        assert_eq!(Solution::number_of_ways("SSPPSPS".to_string()), 3);
        assert_eq!(Solution::number_of_ways("PPSPSP".to_string()), 1);
        assert_eq!(Solution::number_of_ways("S".to_string()), 0);
    }
}
