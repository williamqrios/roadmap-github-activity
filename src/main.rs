use std::process; 
use github_activity::run; 
// todo: import tokio and use the macro tokio main

fn main() {
    if let Err(error) = run() {
        println!("Application error: {error}"); 
        process::exit(1); 
    }
}
