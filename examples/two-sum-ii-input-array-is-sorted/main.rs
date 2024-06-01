struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;

        let max_index = numbers.len() - 1;

        let mut l = 0;
        let mut r = max_index;

        while l < r {
            let l_num = numbers[l];
            let r_num = numbers[r];

            let sum = l_num + r_num;
            match target.cmp(&sum) {
                Ordering::Equal => return vec![(l + 1) as i32, (r + 1) as i32],

                Ordering::Less => r -= 1,
                Ordering::Greater => l += 1,
            }
        }

        unreachable!();
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2,7,11,15], 9));
}
