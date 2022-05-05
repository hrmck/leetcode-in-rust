/*
 * @lc app=leetcode id=1822 lang=rust
 *
 * [1822] Sign of the Product of an Array
 */

// @lc code=start
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for num in nums {
            if num == 0 {
                return 0;
            }

            if num < 0 {
                sign *= -1;
            }
        }
        sign
    }
}
// @lc code=end

