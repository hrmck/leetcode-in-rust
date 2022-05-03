/*
 * @lc app=leetcode id=1281 lang=rust
 *
 * [1281] Subtract the Product and Sum of Digits of an Integer
 */

// @lc code=start
impl Solution {
    pub fn subtract_product_and_sum(mut n: i32) -> i32 {
        let mut sum: i32 = 0;
        let mut product: i32 = 1;
        while n != 0 {
            sum += n % 10;
            product *= n % 10;
            n /= 10;
        }
        product - sum
    }
}
// @lc code=end
