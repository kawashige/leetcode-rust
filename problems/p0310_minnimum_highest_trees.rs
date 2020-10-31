pub struct Solution {}

use std::collections::HashSet;
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if edges.is_empty() {
            return vec![0];
        }

        let mut matrix = vec![HashSet::new(); n as usize];
        for i in 0..edges.len() {
            matrix[edges[i][0] as usize].insert(edges[i][1] as usize);
            matrix[edges[i][1] as usize].insert(edges[i][0] as usize);
        }

        let mut targets = (0..matrix.len())
            .filter(|i| matrix[*i].len() == 1)
            .collect::<Vec<usize>>();

        let mut remaining = matrix.len();
        while 2 < remaining {
            remaining -= targets.len();
            let mut new_targets = Vec::new();
            for i in targets {
                let j = *matrix[i].iter().next().unwrap();
                matrix[j].remove(&i);
                if matrix[j].len() == 1 {
                    new_targets.push(j);
                }
            }
            targets = new_targets;
        }

        targets.into_iter().map(|i| i as i32).collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0310() {
        assert_eq!(
            vec![1],
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]])
        );
        assert_eq!(
            vec![1, 2],
            Solution::find_min_height_trees(
                7,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 4],
                    vec![3, 5],
                    vec![4, 6]
                ]
            )
        );
        assert_eq!(
            vec![3, 4],
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            )
        );
        assert_eq!(vec![0], Solution::find_min_height_trees(1, vec![]));
        assert_eq!(
            vec![0, 1],
            Solution::find_min_height_trees(2, vec![vec![0, 1]])
        );
        assert_eq!(
            vec![2, 8],
            Solution::find_min_height_trees(
                147,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![0, 3],
                    vec![3, 4],
                    vec![2, 5],
                    vec![5, 6],
                    vec![1, 7],
                    vec![2, 8],
                    vec![0, 9],
                    vec![1, 10],
                    vec![8, 11],
                    vec![8, 12],
                    vec![6, 13],
                    vec![11, 14],
                    vec![12, 15],
                    vec![9, 16],
                    vec![14, 17],
                    vec![4, 18],
                    vec![4, 19],
                    vec![15, 20],
                    vec![20, 21],
                    vec![2, 22],
                    vec![8, 23],
                    vec![14, 24],
                    vec![15, 25],
                    vec![21, 26],
                    vec![18, 27],
                    vec![21, 28],
                    vec![24, 29],
                    vec![14, 30],
                    vec![27, 31],
                    vec![20, 32],
                    vec![21, 33],
                    vec![29, 34],
                    vec![27, 35],
                    vec![0, 36],
                    vec![19, 37],
                    vec![3, 38],
                    vec![19, 39],
                    vec![26, 40],
                    vec![17, 41],
                    vec![40, 42],
                    vec![24, 43],
                    vec![29, 44],
                    vec![31, 45],
                    vec![8, 46],
                    vec![10, 47],
                    vec![5, 48],
                    vec![33, 49],
                    vec![18, 50],
                    vec![3, 51],
                    vec![17, 52],
                    vec![34, 53],
                    vec![0, 54],
                    vec![2, 55],
                    vec![16, 56],
                    vec![46, 57],
                    vec![53, 58],
                    vec![27, 59],
                    vec![1, 60],
                    vec![21, 61],
                    vec![44, 62],
                    vec![9, 63],
                    vec![40, 64],
                    vec![12, 65],
                    vec![63, 66],
                    vec![27, 67],
                    vec![62, 68],
                    vec![7, 69],
                    vec![24, 70],
                    vec![34, 71],
                    vec![26, 72],
                    vec![20, 73],
                    vec![35, 74],
                    vec![48, 75],
                    vec![73, 76],
                    vec![65, 77],
                    vec![0, 78],
                    vec![6, 79],
                    vec![74, 80],
                    vec![70, 81],
                    vec![39, 82],
                    vec![23, 83],
                    vec![28, 84],
                    vec![56, 85],
                    vec![56, 86],
                    vec![49, 87],
                    vec![8, 88],
                    vec![56, 89],
                    vec![14, 90],
                    vec![9, 91],
                    vec![43, 92],
                    vec![65, 93],
                    vec![84, 94],
                    vec![75, 95],
                    vec![28, 96],
                    vec![5, 97],
                    vec![49, 98],
                    vec![94, 99],
                    vec![8, 100],
                    vec![92, 101],
                    vec![93, 102],
                    vec![77, 103],
                    vec![83, 104],
                    vec![63, 105],
                    vec![61, 106],
                    vec![32, 107],
                    vec![54, 108],
                    vec![44, 109],
                    vec![29, 110],
                    vec![68, 111],
                    vec![110, 112],
                    vec![12, 113],
                    vec![49, 114],
                    vec![31, 115],
                    vec![53, 116],
                    vec![53, 117],
                    vec![39, 118],
                    vec![90, 119],
                    vec![2, 120],
                    vec![114, 121],
                    vec![69, 122],
                    vec![110, 123],
                    vec![120, 124],
                    vec![90, 125],
                    vec![121, 126],
                    vec![61, 127],
                    vec![9, 128],
                    vec![47, 129],
                    vec![123, 130],
                    vec![25, 131],
                    vec![89, 132],
                    vec![77, 133],
                    vec![127, 134],
                    vec![12, 135],
                    vec![63, 136],
                    vec![84, 137],
                    vec![101, 138],
                    vec![98, 139],
                    vec![40, 140],
                    vec![112, 141],
                    vec![19, 142],
                    vec![55, 143],
                    vec![88, 144],
                    vec![49, 145],
                    vec![95, 146]
                ]
            )
        );
    }
}
