pub mod simu {
    use std::{backtrace::Backtrace, collections::VecDeque};
    use crate::trades::trades::Trade;

    const BASE: f64 = 1000.;


    pub struct Agent {
        base: f64,  
        current: f64,

    }

    pub fn read_trades(trades: &VecDeque<Trade>) {
        let l = trades.len() - 1;
        for i in 0..l {
            let trade0 = trades.get(i).unwrap();        // dernier prix 
            let trade1 = trades.get(i + 1).unwrap();    // avant-dernier prix
            let price0 = trade0.price;
            let price1 = trade1.price;
            // if price1 > 0. {
            //     let variation = (price0 - price1) / price1;
            // }
        }
    }
    
    pub start_agents() {
        let neutre = Agent {
            base: BASE,
            current: BASE,
        };
        
    }







}