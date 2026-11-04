pub struct Solution {}

struct DualHeap {
    left: std::collections::BinaryHeap<i32>,
    right: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
    delayed: std::collections::HashMap<i32, i32>,
    left_size: i32,
    right_size: i32,
    sum_l: i64,
    sum_r: i64,
}

impl DualHeap {
    fn new() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            delayed: Default::default(),
            left_size: 0,
            right_size: 0,
            sum_l: 0,
            sum_r: 0,
        }
    }

    fn prune_left(&mut self) {
        while let Some(&num) = self.left.peek() {
            let count = self.delayed.entry(num).or_default();
            if *count == 0 {
                break;
            }
            *count -= 1;
            self.left.pop();
        }
    }

    fn prune_right(&mut self) {
        while let Some(&std::cmp::Reverse(num)) = self.right.peek() {
            let count = self.delayed.entry(num).or_default();
            if *count == 0 {
                break;
            }
            *count -= 1;
            self.right.pop();
        }
    }

    fn balance(&mut self) {
        if self.left_size > self.right_size + 1 {
            let num = self.left.pop().unwrap();
            self.right.push(std::cmp::Reverse(num));
            self.sum_r += i64::from(num);
            self.sum_l -= i64::from(num);
            self.right_size += 1;
            self.left_size -= 1;
            self.prune_left();
        } else if self.right_size > self.left_size {
            let num = self.right.pop().unwrap().0;
            self.left.push(num);
            self.sum_l += i64::from(num);
            self.sum_r -= i64::from(num);
            self.left_size += 1;
            self.right_size -= 1;
            self.prune_right();
        }
    }

    fn add(&mut self, num: i32) {
        if self.left.is_empty() || num <= *self.left.peek().unwrap() {
            self.left.push(num);
            self.left_size += 1;
            self.sum_l += i64::from(num);
        } else {
            self.right.push(std::cmp::Reverse(num));
            self.right_size += 1;
            self.sum_r += i64::from(num);
        }
        self.balance();
    }

    fn remove(&mut self, num: i32) {
        *self.delayed.entry(num).or_default() += 1;
        if num <= *self.left.peek().unwrap() {
            self.left_size -= 1;
            self.sum_l -= i64::from(num);
            if num == *self.left.peek().unwrap() {
                self.prune_left();
            }
        } else {
            self.right_size -= 1;
            self.sum_r -= i64::from(num);
            if num == self.right.peek().unwrap().0 {
                self.prune_right();
            }
        }
        self.balance();
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;
        let mut heaps = DualHeap::new();
        let mut ops = vec![0_i64; n];
        for i in 0..n {
            heaps.add(nums[i]);
            if i >= x {
                heaps.remove(nums[i - x]);
            }
            if i >= x - 1 {
                let mid = i64::from(*heaps.left.peek().unwrap());
                ops[i] =
                    heaps.sum_r - heaps.sum_l + mid * i64::from(heaps.left_size - heaps.right_size);
            }
        }

        let mut dp = vec![vec![i64::MAX; k + 1]; n];
        for row in &mut dp {
            row[0] = 0;
        }
        for j in 1..=k {
            if j == 1 {
                dp[x - 1][j] = ops[x - 1];
            } else {
                dp[j * x - 1][j] = dp[(j - 1) * x - 1][j - 1] + ops[j * x - 1];
            }
        }
        for j in 1..=k {
            for i in j * x..n {
                dp[i][j] = (dp[i - x][j - 1] + ops[i]).min(dp[i - 1][j]);
            }
        }
        dp[n - 1][k]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3505() {
        assert_eq!(
            Solution::min_operations(vec![5, -2, 1, 3, 7, 3, 6, 4, -1], 3, 2),
            8
        );
        assert_eq!(Solution::min_operations(vec![9, -2, -2, -2, 1, 5], 2, 2), 3);
    }
}

fn main() {}
