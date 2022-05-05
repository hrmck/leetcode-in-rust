/*
 * @lc app=leetcode id=1790 lang=rust
 *
 * [1790] Check if One String Swap Can Make Strings Equal
 */

// @lc code=start
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }
        let mut swap_pair = None;
        let mut count: i8 = 0;
        for (s1_char, s2_char) in s1.chars().zip(s2.chars()) {
            if s1_char == s2_char {
                continue;
            }

            count += 1;
            if count > 2 {
                return false;
            }

            match swap_pair {
                Some((s2_target, s1_target)) => {
                    if !(s1_char == s1_target && s2_char == s2_target) {
                        return false;
                    }
                }
                None => swap_pair = Some((s1_char, s2_char)),
            }
        }
        count != 1
    }
}
// @lc code=end
