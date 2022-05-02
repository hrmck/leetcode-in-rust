/*
 * @lc app=leetcode id=1523 lang=rust
 *
 * [1523] Count Odd Numbers in an Interval Range
 */

// @lc code=start
fn is_odd(num: i32) -> bool {
    num % 2 == 1
}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let diff: i32 = high - low;        
        if is_odd(diff) {
            (diff + 1) / 2
        } else {
            if is_odd(low) {
                diff / 2 + 1
            } else {
                diff / 2
            }
        }
    }
}
// @lc code=end

