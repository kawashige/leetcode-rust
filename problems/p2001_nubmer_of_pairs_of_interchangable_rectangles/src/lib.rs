use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut map = HashMap::new();
        let mut count = 0;
        for rectangle in rectangles {
            let val = rectangle[0] as f64 / rectangle[1] as f64;
            let key = format!("{:.20}", val);
            count += map.get(&key).unwrap_or(&0);
            *map.entry(key).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2001() {
        assert_eq!(
            Solution::interchangeable_rectangles(vec![
                vec![15, 4],
                vec![21, 33],
                vec![78, 56],
                vec![71, 33],
                vec![13, 27],
                vec![39, 21],
                vec![79, 52],
                vec![37, 3],
                vec![49, 71],
                vec![67, 58],
                vec![25, 34],
                vec![24, 12],
                vec![35, 38],
                vec![53, 63],
                vec![74, 56],
                vec![3, 29],
                vec![30, 23],
                vec![9, 22],
                vec![18, 44],
                vec![28, 7],
                vec![3, 51],
                vec![45, 69],
                vec![40, 10],
                vec![64, 14],
                vec![44, 54],
                vec![5, 49],
                vec![68, 22],
                vec![28, 35],
                vec![80, 66],
                vec![45, 1],
                vec![50, 38],
                vec![42, 73],
                vec![36, 32],
                vec![5, 10],
                vec![55, 8],
                vec![10, 40],
                vec![40, 50],
                vec![55, 28],
                vec![4, 51],
                vec![18, 22],
                vec![26, 65],
                vec![33, 17],
                vec![46, 54],
                vec![43, 59],
                vec![5, 76],
                vec![14, 33],
                vec![77, 4],
                vec![19, 5],
                vec![59, 22],
                vec![21, 26],
                vec![59, 42],
                vec![6, 11],
                vec![63, 69],
                vec![26, 45],
                vec![3, 42],
                vec![21, 58],
                vec![65, 55],
                vec![69, 70],
                vec![64, 61],
                vec![76, 25],
                vec![2, 72],
                vec![43, 77],
                vec![29, 68],
                vec![65, 33],
                vec![48, 44],
                vec![35, 15],
                vec![45, 38],
                vec![54, 73],
                vec![71, 75],
                vec![28, 73],
                vec![80, 79],
                vec![74, 42],
                vec![11, 3],
                vec![35, 13],
                vec![31, 9],
                vec![80, 24],
                vec![65, 64],
                vec![22, 69],
                vec![66, 27],
                vec![70, 29],
                vec![58, 2],
                vec![39, 17]
            ]),
            3
        );
        assert_eq!(
            Solution::interchangeable_rectangles(vec![
                vec![4, 8],
                vec![3, 6],
                vec![10, 20],
                vec![15, 30]
            ]),
            6
        );
        assert_eq!(
            Solution::interchangeable_rectangles(vec![vec![4, 5], vec![7, 8]]),
            0
        );
    }
}
