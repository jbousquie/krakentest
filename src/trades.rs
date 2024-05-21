
use std::{fs::OpenOptions, io::Write};

use serde::{Deserialize, Serialize};
use serde_json::Result;

// datetime from rfc3339 string : https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_rfc3339


#[derive(Serialize, Deserialize, Debug)]
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

pub fn process_msg(payload: &str) -> Vec<Trade>{
    let msg: Result<Msg> = serde_json::from_str(payload);
    let trades: Vec<Trade> = match msg {
        Ok(msg) => msg.data,
        Err(_) => {
            vec!()
        }
    };
    //println!("{:?}", trades);
    record_trades(&trades);
    trades
}

pub fn record_trades(trades: &Vec<Trade>)  {
    let filename = "trades.csv";
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
            // Comparaison avec le dernier timestamp stocké
            // let datetime_ts = match DateTime::parse_from_rfc3339(timestamp) {
            //     Ok(ts) => ts.timestamp(),
            //     Err(_) => 0,
            // };
            let line = format!("{},{},{},{},{},{}\n", symbol, side, price, qty, ord_type, timestamp);
            write_file(&filename, &line);
        }
    }
}

fn write_file(file_path: &str, line: &str) {
    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open(file_path)
    .expect("erreur ouverture fichier des trades");
    file.write(line.as_bytes()).expect("erreur écriture fichier des trades");
}