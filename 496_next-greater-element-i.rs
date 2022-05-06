/*
 * @lc app=leetcode id=496 lang=rust
 *
 * [496] Next Greater Element I
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        let mut sub: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        const DEFAULT_VALUE: i32 = -1;

        for num in nums2 {
            while !sub.is_empty() && sub.last().cloned() < Some(num) {
                map.insert(sub.pop().unwrap(), num);
            }
            sub.push(num);
        }
        for num in nums1 {
            results.push(**(map.get(&num).get_or_insert(&DEFAULT_VALUE)));
        }
        results
    }
}
// @lc code=end
