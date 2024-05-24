struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut origin = x;
        let mut result = 0;
        let is_negative = x < 0;

        if is_negative {
            origin = -origin;
        }

        while origin > 0 {
            if result > i32::MAX / 10 {
                return 0;
            }
            result *= 10;
            result += origin % 10;
            origin /= 10;
        }

        if is_negative {
            result = -result;
        }
        return result;
    }
}

fn main() {
    println!("{}", Solution::reverse(1463847412));
}