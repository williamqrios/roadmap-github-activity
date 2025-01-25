use std::process; 
use github_activity::run; 

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        println!("Application error: {error}"); 
        process::exit(1); 
    }
}
