/*
 * @lc app=leetcode id=202 lang=rust
 *
 * [202] Happy Number
 */

// @lc code=start
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        let mut num = n;
        let mut sum: i32 = 0;
        let mut visited_nums: Vec<i32> = Vec::new();

        loop {
            visited_nums.push(num);
            while num != 0 {
                sum += (num % 10) * (num % 10);
                num /= 10;
            }
            if sum == 1 {
                return true;
            } else if visited_nums.iter().any(|&visited_nums| visited_nums == sum) {
                return false;
            } else {
                num = sum;
                sum = 0;
            }
        }
    }
}
// @lc code=end
