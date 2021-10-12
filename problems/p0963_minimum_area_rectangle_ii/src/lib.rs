pub struct Solution {}

impl Solution {
    pub fn min_area_free_rect(mut points: Vec<Vec<i32>>) -> f64 {
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        let mut min = std::f64::MAX;

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                for k in (j + 1)..points.len() {
                    for l in (k + 1)..points.len() {
                        let v12 = (points[j][0] - points[i][0], points[j][1] - points[i][1]);
                        let v13 = (points[k][0] - points[i][0], points[k][1] - points[i][1]);
                        let v14 = (points[l][0] - points[i][0], points[l][1] - points[i][1]);

                        if v12.0 * v13.0 + v12.1 * v13.1 == 0
                            && (v12.0 + v13.0, v12.1 + v13.1) == v14
                        {
                            let area = (((v12.0 * v12.0 + v12.1 * v12.1) as i64
                                * (v13.0 * v13.0 + v13.1 * v13.1) as i64)
                                as f64)
                                .sqrt();

                            if area < min {
                                min = area;
                            }
                        }
                    }
                }
            }
        }

        if min == std::f64::MAX {
            0.0
        } else {
            min
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_963() {
        assert_eq!(
            Solution::min_area_free_rect(vec![vec![1, 2], vec![2, 1], vec![1, 0], vec![0, 1]]),
            2.0
        );
        assert_eq!(
            Solution::min_area_free_rect(vec![
                vec![0, 1],
                vec![2, 1],
                vec![1, 1],
                vec![1, 0],
                vec![2, 0]
            ]),
            1.0
        );
        assert_eq!(
            Solution::min_area_free_rect(vec![
                vec![0, 3],
                vec![1, 2],
                vec![3, 1],
                vec![1, 3],
                vec![2, 1]
            ]),
            0.0
        );
        assert_eq!(
            Solution::min_area_free_rect(vec![
                vec![3, 1],
                vec![1, 1],
                vec![0, 1],
                vec![2, 1],
                vec![3, 3],
                vec![3, 2],
                vec![0, 2],
                vec![2, 3]
            ]),
            2.0
        );
        assert_eq!(
            Solution::min_area_free_rect(vec![
                vec![24420, 16685],
                vec![20235, 25520],
                vec![14540, 20845],
                vec![20525, 14500],
                vec![16876, 24557],
                vec![24085, 23720],
                vec![25427, 18964],
                vec![21036, 14573],
                vec![24420, 23315],
                vec![22805, 24760],
                vec![21547, 25304],
                vec![16139, 23952],
                vec![21360, 14645],
                vec![24715, 17120],
                vec![19765, 25520],
                vec![19388, 25491],
                vec![22340, 25005],
                vec![25520, 19765],
                vec![25365, 21320],
                vec![23124, 15443],
                vec![20845, 14540],
                vec![24301, 16532],
                vec![16685, 24420],
                vec![25100, 17875],
                vec![22125, 25100],
                vec![15699, 23468],
                vec![14592, 21131],
                vec![25460, 19155],
                vec![17837, 25084],
                vec![23468, 24301],
                vec![25460, 20845],
                vec![18453, 25304],
                vec![21131, 14592],
                vec![22805, 15240],
                vec![19475, 25500],
                vec![15443, 23124],
                vec![25355, 21360],
                vec![15285, 22880],
                vec![20000, 25525],
                vec![24085, 16280],
                vec![22163, 25084],
                vec![22880, 15285],
                vec![14916, 22163],
                vec![16280, 24085],
                vec![24875, 17400],
                vec![22600, 24875],
                vec![14573, 21036],
                vec![25427, 21036],
                vec![17120, 24715],
                vec![25500, 19475]
            ]),
            2141490.00000
        );
    }
}
