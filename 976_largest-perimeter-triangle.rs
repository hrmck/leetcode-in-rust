/*
 * @lc app=leetcode id=976 lang=rust
 *
 * [976] Largest Perimeter Triangle
 */

// @lc code=start
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for ix in (2..nums.len()).rev() {
            if nums[ix - 2] + nums[ix - 1] > nums[ix] {
                return nums[ix - 2] + nums[ix - 1] + nums[ix];
            }
        }
        0
    }
}
// @lc code=end
