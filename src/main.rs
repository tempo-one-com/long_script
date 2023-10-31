use std::{time::Duration, env};
use tokio::time::sleep;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nb_seconds = env::args().nth(1)
        .map(|x| x.parse::<u64>())
        .unwrap_or_else(|| { 
            std::process::exit(64); //mauvais usage
        })?;

    sleep(Duration::from_secs(nb_seconds)).await;        
    println!("durée d'execution: {}s", nb_seconds);

    Ok(())
}
