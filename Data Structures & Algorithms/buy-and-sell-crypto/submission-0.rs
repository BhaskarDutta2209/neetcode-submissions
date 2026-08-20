impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut low_price = i32::MAX;

        for price in prices.iter() {
            if *price >= low_price {
                let p = *price - low_price;
                if p > profit {
                    profit = p;
                }
            } else {
                low_price = *price;
            }
        }

        profit
    }
}
