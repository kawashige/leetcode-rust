pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => vec![1],
            1 => vec![1, 1],
            _ => {
                let mut row = vec![1, 1];
                for i in 2..=row_index {
                    let mut next_row = vec![1];
                    for j in 0..(i - 1) {
                        next_row.push(row[j as usize] + row[j as usize + 1]);
                    }
                    next_row.push(1);
                    row = next_row
                }
                row
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day12() {
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], Solution::get_row(5));
    }
}
