struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0;
        let mut is_negative = false;
        let mut is_reading_digit = false;

        for ch in s.chars() {
            if !is_reading_digit {
                match ch {
                    ' ' => continue,
                    '+' => {
                        is_negative = false;
                        is_reading_digit = true;
                        continue;
                    }
                    '-' => {
                        is_negative = true;
                        is_reading_digit = true;
                        continue;
                    }
                    _ => ()
                }
            }
            

            if ch.is_ascii_digit() {
                if result > i32::MAX / 10 {
                    if is_negative {
                        return i32::MIN;
                    } else {
                        return i32::MAX;
                    }
                }

                result *= 10;
                result += ch.to_digit(10).unwrap() as i32;
            } else {
                break;
            }
        }

        if is_negative {
            result = -result;
        }

        return result;
    }
}

fn main() {
    println!("{}", Solution::my_atoi(String::from("00000-42a")));
}