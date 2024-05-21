pub mod trades {

    use std::{collections::VecDeque, fs::OpenOptions, io::Write};

    use serde::{Deserialize, Serialize};
    use serde_json::Result;

    // datetime from rfc3339 string : https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_rfc3339

    

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Trade {
        symbol: String,
        side: String,
        price: f64,
        qty: f64,
        ord_type: String,
        timestamp: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Msg {
        // on ne récupère que l'attribut data du message
        data: Vec<Trade>
    }

    pub struct Trades<'a> {
        pub file_max_len: usize,
        pub mem_max_len: usize,
        pub list: VecDeque<Trade>,
        pub filename: &'a str,
    }

    impl Trades <'_>{
        pub fn process_msg(&mut self, payload: &str) {
            let msg: Result<Msg> = serde_json::from_str(payload);
            let trades: Vec<Trade> = match msg {
                Ok(msg) => msg.data,
                Err(_) => {
                    vec!()
                }
            };
            self.store_trades(&trades);
            self.record_trades(&trades);
        }

        pub fn store_trades(&mut self, trades: &Vec<Trade>) {
            let l = trades.len();
            if l > 0 {
                for i in 0..l {
                    let trade = trades[i].clone();
                    self.list.push_front(trade);
                }
                if self.list.len() > self.mem_max_len {
                    let default = Self::default_trade();
                    self.list.resize(self.mem_max_len, default);
                }
            }
        }
    
        pub fn record_trades(&self, trades: &Vec<Trade>)  {
            let l = trades.len();
            if l > 0 {                       
                for i in 0..l {
                    let trade = &trades[i];
                    let symbol = &trade.symbol;
                    let side  = &trade.side;
                    let price = &trade.price;
                    let qty = &trade.qty;
                    let ord_type = &trade.ord_type;
                    let timestamp = &trade.timestamp;
                    let line = format!("{},{},{},{},{},{}\n", symbol, side, price, qty, ord_type, timestamp);
                    self.write_file(self.filename, &line);
                }
            }
        }
    
        fn write_file(&self, file_path: &str, line: &str) {
            let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("erreur ouverture fichier des trades");
            file.write(line.as_bytes()).expect("erreur écriture fichier des trades");
        }

        fn default_trade() -> Trade {
            Trade {
                symbol: "".into(),
                side: "".into(),
                price: 0.,
                qty: 0.,
                ord_type: "".into(),
                timestamp: "".into(),
            }
        }
    }

}