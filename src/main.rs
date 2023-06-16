use std::env;
use std::format;
use std::process::exit;
use dotenv::dotenv;
use url::Url;
use tungstenite::connect;
use serde_json;

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

    let parsed: serde_json::Value = serde_json::from_str(message.to_text().unwrap()).expect("Can't parse JSON");

    if parsed["error"].is_string() {
        println!("Authentication error");
        exit(1);
    }

    let exposure = parsed["exposure"].as_f64().expect("Expected exposure");
    let leverage_deribit = parsed["leverage_deribit"].as_f64().expect("Expected Deribit leverage");
    let leverage_bybit = parsed["leverage_bybit"].as_f64().expect("Expected Bybit leverage");

    println!("Exposure: {:.8}\nLeverage: {:.2} {:.2}", exposure, leverage_deribit, leverage_bybit);
}
