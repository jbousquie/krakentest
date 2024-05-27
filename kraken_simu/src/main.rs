use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};


const BASE: f64 = 1000.;    // mise de base des agents
const INTERVAL: u64 = 20;        // intervalle en secondes d'action des agents
const FILENAME: &str = "trades.csv";

struct Agent {
    start: f64,
    current: f64,
}

impl Agent {
    fn trade(&mut self, trades: &Vec<Trade>) {
        let l = trades.len();
        if l >= 2 {
            let start = trades[0].price;
            let end = trades[l - 1].price;
            let variation = (end - start) / start ;
            for i in 0..l {
                let trade = &trades[i];
            }
            let current = self.start * (1. + variation);
            self.current = current;
            println!("{current}");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub symbol: String,
    pub side: String,
    pub price: f64,
    pub qty: f64,
    pub ord_type: String,
    pub timestamp: String,
}


// Récupère les trades du fichier et les charges dans la liste de self
pub fn get_trades_from_file(filename: &str) -> Vec<Trade> {
    let content = fs::read_to_string(filename).expect("erreur de lecture du fichier de trades");
    let lines: Vec<&str> = content.split('\n').collect();
    let mut trades: Vec<Trade> = vec![];
    let l = lines.len() - 1;  // la dernière ligne du fichier est vide
    if l > 2 {
        for i in 0..l {
            let line = lines[i];
            let trade = trade_from_line(line);
            trades.push(trade);
        }
    }
    trades
}

// Fabrique un objet Trade à partir d'une ligne du fichier
fn trade_from_line(line: &str) -> Trade {
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

fn simulate() {
    let trades = get_trades_from_file(FILENAME);
    let mut agent1 = Agent { start: BASE, current: BASE };
    agent1.trade(&trades);

}

fn main() {
    let interval = Duration::from_secs(INTERVAL);
    let mut next_time = Instant::now() + interval;
    loop {
        simulate();
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}