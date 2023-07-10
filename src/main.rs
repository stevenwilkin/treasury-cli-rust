use std::env;
use std::format;
use std::process::exit;
use dotenv::dotenv;
use url::Url;
use tungstenite::connect;
use serde_json;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct Message {
    #[serde(default)]
    error: String,
    #[serde(default)]
    exposure: f64,
    #[serde(default)]
    leverage_deribit: f64,
    #[serde(default)]
    leverage_bybit: f64,
}

fn main() {
    dotenv().ok();

    let url = match env::var("TREASURY_WS_URL") {
        Ok(u) => u,
        Err(_) => {
            println!("TREASURY_WS_URL not set");
            exit(1);
        }
    };

    let auth_token = match env::var("TREASURY_AUTH_TOKEN") {
        Ok(at) => at,
        Err(_) => {
            println!("TREASURY_AUTH_TOKEN not set");
            exit(1);
        }
    };

    let mut socket = match connect(Url::parse(&url).unwrap()) {
        Ok(s) => s.0,
        Err(_) => {
            println!("Could not connect to {}", url);
            exit(1);
        }
    };

    let auth_message = format!("{{\"auth\":\"{}\"}}", auth_token);
    if socket.write_message(tungstenite::Message::text(auth_message)).is_err() {
        println!("Error sending auth token");
        exit(1);
    }

    loop {
        let message = match socket.read_message() {
            Ok(m) => m,
            Err(_) => {
                println!("Error reading message");
                exit(1);
            }
        };

        if !message.is_text() {
            println!("Unexpected message type");
            exit(1);
        }

        let m: Message = serde_json::from_str(message.to_text().unwrap()).unwrap();

        if m.error != "" {
            println!("Authentication error");
            exit(1);
        }

        println!("\x1b[2J\x1b[H\x1b[?25l");
        println!("  Exposure: {:.8}", m.exposure);
        println!("  Leverage: {:.2} {:.2}", m.leverage_deribit, m.leverage_bybit);
    }
}
