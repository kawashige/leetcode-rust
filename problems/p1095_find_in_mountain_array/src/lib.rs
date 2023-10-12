pub struct MountainArray {
    arr: Vec<i32>,
}
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }
    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}

pub struct Solution {}

impl Solution {
    pub fn find_top(target: i32, mountainArr: &MountainArray, l: i32, r: i32) -> i32 {
        let mid = (l + r) / 2;
        if mid == 0 {
            return Self::find_top(target, mountainArr, mid + 1, r);
        } else if mid == mountainArr.length() - 1 {
            return Self::find_top(target, mountainArr, l, mid - 1);
        }
        let arr = [
            mountainArr.get(mid - 1),
            mountainArr.get(mid),
            mountainArr.get(mid + 1),
        ];
        if arr[0] < arr[1] && arr[1] > arr[2] {
            mid
        } else if arr[0] < arr[1] && arr[1] < arr[2] {
            Self::find_top(target, mountainArr, mid + 1, r)
        } else {
            Self::find_top(target, mountainArr, l, mid - 1)
        }
    }

    pub fn find_left(target: i32, mountainArr: &MountainArray, l: i32, r: i32) -> i32 {
        let l_val = mountainArr.get(l);
        let r_val = mountainArr.get(r);
        if l_val == target {
            return l;
        } else if r_val == target {
            return r;
        } else if target < l_val || r_val < target {
            return -1;
        }
        let mut ng = l;
        let mut ok = r;

        while ng + 1 < ok {
            let mid = (ng + ok) / 2;
            if target <= mountainArr.get(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }

    pub fn find_right(target: i32, mountainArr: &MountainArray, l: i32, r: i32) -> i32 {
        let l_val = mountainArr.get(l);
        let r_val = mountainArr.get(r);
        if l_val == target {
            return l;
        } else if r_val == target {
            return r;
        } else if target < r_val || l_val < target {
            return -1;
        }
        let mut ok = l;
        let mut ng = r;

        while ok + 1 < ng {
            let mid = (ng + ok) / 2;
            if target <= mountainArr.get(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        let top = Self::find_top(target, mountainArr, 0, mountainArr.length() - 1);
        let left = Self::find_left(target, mountainArr, 0, top);
        if left != -1 && mountainArr.get(left) == target {
            return left;
        }
        let right = Self::find_right(target, mountainArr, top, mountainArr.length() - 1);
        if right != -1 && mountainArr.get(right) == target {
            return right;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1095() {
        assert_eq!(
            Solution::find_in_mountain_array(
                0,
                &MountainArray {
                    arr: vec![3, 5, 3, 2, 0]
                }
            ),
            4
        );
        assert_eq!(
            Solution::find_in_mountain_array(2, &MountainArray { arr: vec![1, 5, 2] }),
            2
        );
        assert_eq!(
            Solution::find_in_mountain_array(
                3,
                &MountainArray {
                    arr: vec![1, 2, 3, 4, 5, 3, 1]
                }
            ),
            2
        );
        assert_eq!(
            Solution::find_in_mountain_array(
                3,
                &MountainArray {
                    arr: vec![0, 1, 2, 4, 2, 1]
                }
            ),
            -1
        );
    }
}
