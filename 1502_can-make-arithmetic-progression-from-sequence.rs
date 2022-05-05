/*
 * @lc app=leetcode id=1502 lang=rust
 *
 * [1502] Can Make Arithmetic Progression From Sequence
 */

// @lc code=start
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        if arr.len() <= 2 {
            return true;
        }
        arr.sort_unstable();
        for ix in (2..arr.len()) {
            if arr[ix - 2] - arr[ix - 1] != arr[ix - 1] - arr[ix] {
                return false;
            }
        }
        true
    }
}
// @lc code=end
