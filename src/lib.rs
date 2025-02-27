/// A library to find the length of the longest subsequence with decreasing adjacent differences.
pub struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut dp = vec![1; n]; // dp[i] stores the length of the longest subsequence ending at i
        let mut max_len = 1;

        for i in 1..n {
            for j in 0..i {
                let diff_i_j = (nums[i] - nums[j]).abs();
                let diff_j_prev = if j > 0 { (nums[j] - nums[j - 1]).abs() } else { i32::MAX };

                if j == 0 || diff_i_j < diff_j_prev {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_len = max_len.max(dp[i]);
        }

        max_len
    }
}