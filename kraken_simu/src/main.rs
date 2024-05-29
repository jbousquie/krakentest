use std::fs;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::prelude::*;


const BASE: f64 = 1000.;            // mise de base des agents
const INTERVAL: u64 = 20;           // intervalle en secondes d'action des agents
const FILENAME: &str = "trades.csv";
const FEE: f64 = 0.0012;
const MARGE: f64 = 0.0003;          // marge +/- en pourcentage à appliquer au prix courant du marché pour être sûr de vendre/acheter
const THR_GAIN: f64 = 0.0003;
const THR_LOSS: f64 = 0.0001;

struct Agent {
    start: f64,             // montant investi en USD
    current: f64,           // solde courant en USD
    buyer: bool,            // mode acheteur
    crypto: f64,            // solde courant en crypto
    last: f64,              // dernier prix payé
    thr_gain: f64,          // seuil de gain net en % avant achat
    thr_loss: f64,          // seuil de perte nette avant vente en %
}

impl Agent {
    fn trade(&mut self, trades: &Vec<Trade>) -> f64 {
        let l = trades.len();
        if l >= 2 {
            let mode = "trade";
            let start = trades[0].price;
            self.buy(start, mode);         
            
            for i in 1..l {
                let price = trades[i].price;
                // my_price prix estimé proposé à la vente
                let gain_price = price * (1. + self.thr_gain); // prix déclencheur de vente avec gain
                let loss_price = price * (1. - self.thr_loss); // prix déclencheur de vente à perte
                if self.buyer && price > trades[i - 1].price {
                    if self.last >= price {
                        self.buy(price, mode);
                    }
                }
                else if price < trades[i - 1].price {
                    if price >= gain_price || price <= loss_price {
                        self.sell(price, mode);
                    }
                }
            }
            // on revend dans tous les cas à la fin
            if !self.buyer {
                let end = trades[l - 1].price;
                self.sell(end, mode);
            }
        }
        let gain = self.current - self.start;
        gain
    }

    // invest : on achète au début et on revend à la fin sans jamais trader
    fn invest(&mut self, trades: &Vec<Trade>) -> f64 {
        self.random(trades, -1.0)
    }

    fn random(&mut self, trades: &Vec<Trade>, fq: f64) -> f64 {
        let l = trades.len();
        if l >= 2 {
            let mode = if fq > 0. { "random" } else { "invest" };
            let mut rng = rand::thread_rng();
            let start = trades[0].price;
            self.buy(start, mode);
            if fq > 0. {
                for i in 1..l {
                    let rdm: f64 = rng.gen();
                    if rdm <= fq {
                        let price = trades[i].price;
                        if self.buyer {
                            self.buy(price, mode);
                        }
                        else {
                            self.sell(price, mode);
                        }
                    }
                }
            }

            // on revend dans tous les cas à la fin
            if !self.buyer {
                let end = trades[l - 1].price;
                self.sell(end, mode);
            }
        }
        let gain = self.current - self.start;
        gain
    }

    fn buy(&mut self, mut price: f64, mode: &str) {
        if self.buyer && price > 0. {
            self.buyer = false;
            println!("\nAchat ({}) : prix marché = {} USD/BTC", mode, price);
            price = price * (1. + MARGE);       // on paye un peu plus cher que le prix pour augmenter la probabilité d'acheter
            self.crypto = self.current / price * (1. - FEE);
            println!("achat de {:.6} BTC pour {:.2} USD au prix net de {:.2} USD/BTC", self.crypto, self.current, price);
            self.current = 0.;
            self.last = price;
            println!("solde courant = {:.6} BTC", self.crypto);
        }
     }

     fn sell(&mut self, mut price: f64, mode: &str) {
        if !self.buyer && price > 0. {
            self.buyer = true;
            println!("\nVente ({}) : prix marché = {} USD/BTC", mode, price);
            price = price * (1. - MARGE);     // on vend un peu moins cher que le prix pour augmenter la probabilité de vendre
            self.current = self.crypto * price * (1. - FEE);
            println!("vente de {:.6} BTC pour {:.2} USD au prix net de {:.2} USD/BTC", self.crypto, self.current, price);
            self.crypto = 0.;
            println!("solde courant = {:.2} USD", self.current);
            let gain = self.current - self.start;
            println!("gain/perte = {:.2} USD", gain);
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
    let mut agent1 = Agent { 
        start: BASE, 
        current: BASE,
        buyer: true,
        crypto: 0.,
        last: 0.,
        thr_gain: THR_GAIN,
        thr_loss: THR_LOSS,
     };
    let mut agent2 = Agent { 
        start: BASE, 
        current: BASE,
        buyer: true,
        crypto: 0.,
        last: 0.,
        thr_gain: THR_GAIN,
        thr_loss: THR_LOSS,
     };
    let mut agent3 = Agent { 
        start: BASE, 
        current: BASE,
        buyer: true,
        crypto: 0.,
        last: 0.,
        thr_gain: THR_GAIN,
        thr_loss: THR_LOSS,
     };
    let rnd = agent3.random(&trades, 0.01);  // trade au hasard à la fréquence fq
    let trd = agent1.trade(&trades);            // trade selon règle simple
    let inv = agent2.invest(&trades);           // achète au début, attend, revend à la fin
     
    println!("\nGains USD :\nrnd = {:.2}\ninv = {:.2}\ntrd = {:.2}", rnd, inv, trd);
    if trades.len() > 2 {
        let deb = trades[0].price;
        let fin = trades[trades.len() - 1].price;
        println!("prix début = {:.2} USD, prix fin  = {:.2} USD, variation = {:.2}", deb, fin, fin - deb);
    }
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