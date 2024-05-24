struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        #[inline]
        fn max(a: i32, b: i32) -> i32 {
            if a > b { a } else { b }
        }
        #[inline]
        fn min(a: i32, b: i32) -> i32 {
            if a > b { b } else { a }
        }

        let mut left_index = 0_usize;
        let mut right_index = height.len() - 1;
        let mut result_area = 0;

        while left_index < right_index {
            let left_height = height[left_index];
            let right_height = height[right_index];

            let actual_height = min(left_height, right_height);
            let width = (right_index - left_index) as i32;
            let area = actual_height * width;

            result_area = max(result_area, area);

            // alternately change left & right index
            if left_height < right_height {
                left_index += 1;
            } else {
                right_index -= 1;
            }
        }

        return result_area;
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![2,3,10,5,7,8,9]))
}