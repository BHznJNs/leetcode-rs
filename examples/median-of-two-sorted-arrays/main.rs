struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut temp1 = nums1;
        let mut temp2 = nums2;

        temp1.append(&mut temp2);
        temp1.sort();

        let temp = temp1;
        let temp_len = temp.len();

        let result =
        if temp_len % 2 == 0 {
            // even
            let middle_two_sum = temp[temp_len / 2 - 1] + temp[temp_len / 2];
            middle_two_sum as f64 / 2_f64
        } else {
            // odd
            temp[temp_len / 2] as f64
        };
        return result;
    }
}

fn main() {
    println!("{}", Solution::find_median_sorted_arrays(vec![1, 3], vec![2, 7]));
}