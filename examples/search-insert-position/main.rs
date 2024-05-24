struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut begin = 0;
        let mut end = nums.len() - 1;

        while begin <= end {
            let mid_index = (begin + end) / 2;
            let mid_value = nums[mid_index];

            if target == mid_value {
                return mid_index as i32;
            } else
            if target > mid_value {
                if begin == mid_index {
                    begin += 1;
                    continue;
                }
                begin = mid_index;
            } else
            if target < mid_value {
                if end == mid_index {
                    if end == 0 {
                        return 0;
                    }

                    end -= 1;
                    continue;
                }
                end = mid_index;
            }
        }

        if begin >= nums.len() {
            return begin as i32;
        }

        return begin as i32;
    }
}

fn main() {
    println!("{}", Solution::search_insert(vec![1, 3, 5, 6], 5))
}