/*
 * @lc app=leetcode id=1779 lang=rust
 *
 * [1779] Find Nearest Point That Has the Same X or Y Coordinate
 */

// @lc code=start
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut point_distance_iterator = points.iter().enumerate()
                                        .filter(|&(i, point)| point[0] == x || point[1] == y)
                                        .map(|(i, point)| vec![i as i32, (point[0] - x).abs() + (point[1] - y).abs()])
                                        .peekable();
        if point_distance_iterator.peek().is_none() {
            return -1;
        }
        let mut point_distance: Vec<Vec<i32>> = point_distance_iterator.collect();
        point_distance.sort_by(|a, b| a[1].cmp(&b[1])); // using unstable will create unnecesary swaps
        point_distance[0][0]
    }
}
// @lc code=end

