/*
 * @lc app=leetcode id=1491 lang=rust
 *
 * [1491] Average Salary Excluding the Minimum and Maximum Salary
 */

// @lc code=start
impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let sum: i32 = salary.iter().sum();
        let max: i32 = *(salary.iter().max().unwrap());
        let min: i32 = *(salary.iter().min().unwrap());
        (sum - max - min) as f64 / (salary.len() as f64 - 2.0)
    }
}
// @lc code=end
