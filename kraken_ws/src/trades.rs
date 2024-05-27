pub mod trades {

    use std::{collections::VecDeque, fs::{self, OpenOptions}, io::Write};

    use serde::{Deserialize, Serialize};
    use serde_json::Result;

    // datetime from rfc3339 string : https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_rfc3339

    

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Trade {
        pub symbol: String,
        pub side: String,
        pub price: f64,
        pub qty: f64,
        pub ord_type: String,
        pub timestamp: String,
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

        // Stocke les trades en mémoire et retaille le tableau
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
    
        // Enregistre les trades des messages WS dans un fichier
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
    
        // Ecrit une ligne de trade dans le fichier
        fn write_file(&self, file_path: &str, line: &str) {
            let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .expect("erreur ouverture fichier des trades");
            file.write(line.as_bytes()).expect("erreur écriture fichier des trades");
        }

        // Récupère les trades du fichier et les charges dans la liste de self
        pub fn get_trades_from_file(&mut self) {
            let content = fs::read_to_string(self.filename).expect("erreur de lecture du fichier de trades");
            let lines: Vec<&str> = content.split('\n').collect();
            let l = lines.len();
            if l > 2 {
                let last = l - 2;  // la dernière ligne du fichier est toujours vide
                let min = std::cmp::min(last + 1, self.mem_max_len);
                for i in 0..min {
                    let line = lines[last - i];
                    self.list.push_back(self.trade_from_line(line));
                }
            }
        }

        // Fabrique un objet Trade à partir d'une ligne du fichier
        fn trade_from_line(&self, line: &str) -> Trade {
            let fields: Vec<&str> = line.split(',').collect();
            Trade {
                symbol: fields[0].to_string(),
                side: fields[1].to_string(),
                price: fields[2].parse::<f64>().unwrap(),
                qty: fields[3].parse::<f64>().unwrap(),
                ord_type: fields[4].to_string(),
                timestamp: fields[5].to_string()
            }
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