use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let original_len = nums.len();
        let mut set = HashSet::new();
        for num in nums {
            set.insert(num);
        }
        return original_len != set.len();
    }
}

fn main() {}
