struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        #[inline]
        fn normal_add(target: &mut i32, ch: char) {
            *target +=
            match ch {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!()
            };
        }

        let mut result = 0;
        let mut last_ch = '\0';

        for ch in s.chars() {

            if last_ch == 'I' {
                if ch == 'V' {
                    result += 3;
                    continue;
                }
                if ch == 'X' {
                    result += 8;
                    continue;
                }
            }

            if last_ch == 'X' {
                if ch == 'L' {
                    result += 30;
                    continue;
                }
                if ch == 'C' {
                    result += 80;
                    continue;
                }
            }

            if last_ch == 'C' {
                if ch == 'D' {
                    result += 300;
                    continue;
                }
                if ch == 'M' {
                    result += 800;
                    continue;
                }
            }

            normal_add(&mut result, ch);
            last_ch = ch;
        }
        return result;
    }
}

fn main() {
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")))
}