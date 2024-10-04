use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut abs_min = prices[0];

        for now_price in prices {
            if now_price < abs_min {
                abs_min = now_price;
            } else if now_price - abs_min > res {
                res = now_price - abs_min;
            }
        }

        res
    }
}
