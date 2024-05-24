struct Solution;
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from('1');
        }

        let sub_case = Self::count_and_say(n - 1);
        let mut chars = sub_case.chars();

        let mut result = String::new();

        let mut last_ch = chars.next().unwrap();
        let mut ch_counter = 1;

        for ch in chars {
            if ch == last_ch {
                ch_counter += 1;
            } else {
                result.push_str(&ch_counter.to_string());
                result.push(last_ch);

                last_ch = ch;
                ch_counter = 1;
            }
        }
        result.push_str(&ch_counter.to_string());
        result.push(last_ch);

        return result;
    }
}

fn main() {
    println!("{}", Solution::count_and_say(4));
}