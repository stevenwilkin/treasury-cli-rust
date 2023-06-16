use std::env;
use std::process::exit;
use dotenv::dotenv;

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

    println!("{}", url);
    println!("{}", auth_token);
}
