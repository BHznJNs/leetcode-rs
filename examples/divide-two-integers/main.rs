struct Solution;
impl Solution {
    pub fn divide(origin_dividend: i32, origin_divisor: i32) -> i32 {
        #[inline]
        fn minus_abs(origin: i32) -> i32 {
            if origin > 0 {
                return -origin;
            } else {
                return origin;
            }
        }

        let mut dividend = origin_dividend;
        let mut divisor = origin_divisor;

        let mut quotient = 0;
        let is_dividend_negative = dividend < 0;
        let is_divisor_negative = divisor < 0;

        // --- --- --- --- ---
        // special conditions|
        // --- --- --- --- ---
        if divisor == -1 {
            if dividend == i32::MIN {
                return i32::MAX;
            } else {
                return -dividend;
            }
        }
        if divisor == 1 {
            return dividend;
        }
        // --- --- --- --- ---
        // special conditions|
        // --- --- --- --- ---

        if !is_dividend_negative { dividend = minus_abs(dividend) };
        if !is_divisor_negative  { divisor  = minus_abs(divisor ) };

        while dividend <= divisor {
            quotient += 1;
            dividend -= divisor;
        }

        // one of dividend and divisor is negative
        if is_dividend_negative ^ is_divisor_negative {
            quotient = -quotient;
        }

        return quotient;
    }
}

fn main() {
    println!("{}", Solution::divide(2147483647, -2147483648));
}