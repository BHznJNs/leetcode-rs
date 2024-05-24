struct Solution;
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        #[inline]
        fn diff(a: i32, b: i32) -> i32 {
            let d = a - b;
            if d >= 0 { d } else { -d }
        }

        let mut index = 0;
        let mut sum = nums[0] + nums[1] + nums[2];
        let mut closest = sum;

        println!("sum: {}", sum);

        while index < nums.len() - 3 {
            sum -= nums[index];
            sum += nums[index + 3];

            println!("sum: {}", sum);

            if diff(sum, target) < diff(closest, target) {
                closest = sum;
            }
            index += 1;
        }
        return closest;
    }
}

fn main() {
    println!("{}", Solution::three_sum_closest(
        vec![4,0,5,-5,3,3,0,-4,-5],
        -2
    ))
}