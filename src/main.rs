// try this example with
// $ cargo run --example wss_client

use std::time::Duration;
use fast_websocket_client::{client, connect, OpCode};
use trades::process_msg;

const PAIR: &str = "BTC/USD";
const TIMEOUT: u64 = 10;    // toutes les combien de secondes, on traite les données reçues
static TIMEOUT_MS: u64 = TIMEOUT * 1000;

mod trades;

#[derive(serde::Serialize)]
struct Params {
    channel: String,
    symbol: Vec<String>,
    snapshot: bool,
}

#[derive(serde::Serialize)]
struct Subscription {
    method: String,
    params: Params,
}

// Envoie le message de souscription au channel "trade"
// https://docs.kraken.com/api/docs/websocket-v2/trade
async fn subscribe(client: &mut client::Online) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let params = Params {
        channel: "trade". to_string(),
        symbol: vec![PAIR.to_string()],
        snapshot: true,
    };
    let data = Subscription {
        method: "subscribe".to_string(),
        params,
    };
    tokio::time::timeout(Duration::from_millis(0), client.send_json(&data)).await??;
    Ok(())
}

// Lance la connexion à l'API WS de Kraken
// Envoie un message de souscription au channel "trade"
// Exécute une boucle infinie de récolte des messages reçus tous les TIMEOUT secondes
// Reconnecte si erreur de connexion
pub fn run_websocket() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let url = "wss://ws.kraken.com/v2";

    let handle = runtime.spawn(async move {
        
        'reconnect_loop: loop {
            let future = connect(url);

            let mut client: client::Online = match future.await {
                Ok(client) => {
                    println!("connected");
                    client
                }
                Err(e) => {
                    eprintln!("Reconnecting from an Error: {e:?}");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    continue;
                }
            };

            // we can modify settings while running.
            // without pong, this app stops in about 15 minutes.(by the binance API spec.)
            client.set_auto_pong(true);

            // add one more example subscription here after connect
            if let Err(e) = subscribe(&mut client).await {
                eprintln!("Reconnecting from an Error: {e:?}");
                let _ = client.send_close(&[]).await;
                tokio::time::sleep(Duration::from_secs(10)).await;
                continue;
            };

            // message processing loop
            loop {
                let message = if let Ok(result) =
                    tokio::time::timeout(Duration::from_millis(TIMEOUT_MS), client.receive_frame()).await
                {
                    match result {
                        Ok(message) => message,
                        Err(e) => {
                            eprintln!("Reconnecting from an Error: {e:?}");
                            let _ = client.send_close(&[]).await;
                            break; // break the message loop then reconnect
                        }
                    }
                } else {
                    //println!("timeout");
                    continue;
                };

                match message.opcode {
                    OpCode::Text => {
                        let payload = match simdutf8::basic::from_utf8(message.payload.as_ref()) {
                            Ok(payload) => payload,
                            Err(e) => {
                                eprintln!("Reconnecting from an Error: {e:?}");
                                let _ = client.send_close(&[]).await;
                                break; // break the message loop then reconnect
                            }
                        };
                        //println!("{payload}");
                        // on ne traite que les messages de trade, commençant par {"channel":"trade"
                        if payload.starts_with("{\"channel\":\"trade\"") {
                            process_msg(payload);
                        }
                    }
                    OpCode::Close => {
                        println!("{:?}", String::from_utf8_lossy(message.payload.as_ref()));
                        break 'reconnect_loop;
                    }
                    _ => {}
                }
            }
        }
    });
    runtime.block_on(handle)?;
    Ok(())
}

fn main() {
    let _ws = run_websocket();

}