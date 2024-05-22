pub mod simu {
    use trades::trades::Trade;

    pub fn read_trades(trades: VecDeque<Trade>) {
        let l = trades.len();
        for i in 0..l {
            let trade = trades[i];
        }
    }
}