pub mod simu {
    use std::collections::VecDeque;


    use crate::trades::trades::Trade;

    pub fn read_trades(trades: &VecDeque<Trade>) {
        let l = trades.len() - 1;
        for i in 0..l {
            let trade0 = trades.get(i).unwrap();
            let trade1 = trades.get(i + 1).unwrap();
            let price0 = trade0.price;
            let price1 = trade1.price;
            if price1 > 0. {
                let variation = (price0 - price1) / price1;
                println!("{} {}", i, variation);
            }
        }
    }
}