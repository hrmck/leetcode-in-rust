/*
 * @lc app=leetcode id=1232 lang=rust
 *
 * [1232] Check If It Is a Straight Line
 */

// @lc code=start
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() <= 2 {
            return true;
        }
        let (mut x1, mut y1) = (coordinates[0][0], coordinates[0][1]);
        let (mut x2, mut y2) = (coordinates[1][0], coordinates[1][1]);
        for ix in 2..coordinates.len() {
            if ((y2 - y1) * (x1 - coordinates[ix][0]) != (y1 - coordinates[ix][1]) * (x2 - x1)) {
                return false;
            }
        }
        true
    }
}
// @lc code=end
