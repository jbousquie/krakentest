use serde::{Deserialize, Serialize};
use serde_json::Result;

// datetime from rfc3339 string : https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_rfc3339

#[derive(Serialize, Deserialize, Debug)]
struct Trade {
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

pub fn process_msg(payload: &str) {
    let msg: Result<Msg> = serde_json::from_str(payload);
    let trades: Vec<Trade> = match msg {
        Ok(msg) => msg.data,
        Err(_) => {
            vec!()
        }
    };
    println!("{:?}", trades);
}