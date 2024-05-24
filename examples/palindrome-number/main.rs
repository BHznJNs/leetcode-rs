struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        if x % 10 == 0 {
            return false;
        }

        let mut origin = x;
        let mut result = 0;

        while origin > 0 {
            result *= 10;
            result += origin % 10;
            origin /= 10;
        }

        return x == result;
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
}