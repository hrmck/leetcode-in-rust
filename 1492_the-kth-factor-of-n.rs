/*
 * @lc app=leetcode id=1492 lang=rust
 *
 * [1492] The kth Factor of n
 */

// @lc code=start
impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count:i32 = 0;
        let mut base:i32 = 1;
        while count <= k && base <= n {
            if n % base == 0 {
                count += 1;
                if count == k {
                    return base;
                }
            }
            base += 1;
        }
        -1
    }
}
// @lc code=end

