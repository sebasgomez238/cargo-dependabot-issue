use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    value: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config {
        name: "example".to_string(),
        value: 42,
    };
    
    println!("Configuration: {:?}", config);
    println!("This is a demo project to reproduce the Dependabot issue with .cargo/config.toml");
    
    Ok(())
}
