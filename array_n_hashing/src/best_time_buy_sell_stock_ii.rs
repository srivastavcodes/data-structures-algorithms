#![allow(dead_code)]

use std::cmp::max;

/// For [7, 1, 5, 3, 6, 4] buy everyday and sell everyday, meaning buy 7 the first day
/// sell it the next day at 1, then see if the profit is positive -> if it is then add
/// in the result else add 0. Always subtract curr - prev because curr is the price we
/// are selling at, so that controls the difference.
///
/// This becomes one cycle, now at the day of selling (price = 1), buy the stock at
/// that price and then sell it the next day repeat the above step.
///
/// This is the general idea, read the code for better understanding.
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;

    for i in 1..prices.len() {
        let curr_max = prices[i] - prices[i - 1];
        max_profit += max(0, curr_max)
    }
    max_profit
}
