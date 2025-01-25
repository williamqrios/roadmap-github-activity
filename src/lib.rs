use std::{error::Error, env};
use reqwest::{header::{USER_AGENT, HeaderMap}, Client};

mod models; 
use models::EventData;

const API_URL: &'static str = "https://api.github.com/users";  

/// Gets the username provided as an argument.
fn parse_args(args: Vec<String>) -> Result<String, String> {
    if args.len() != 2 {
        return Err("Invalid number of arguments. Only one argument is needed (the username).".into()); 
    }
    Ok(args[1].clone())
}

/// Makes the request to the github API using the provided username, checks the status code, and parses the response body into a Rust struct. 
async fn make_request(username: &str) -> Result<Vec<EventData>, Box<dyn Error>> {
    let endpoint = format!("{API_URL}/{username}/events");
    let client = Client::new(); 
    let mut headers = HeaderMap::new(); 
    headers.insert(USER_AGENT, "github-activity-cli".parse().unwrap()); 
    let resp = client.get(endpoint)
        .headers(headers)
        .send()
        .await?; 
    
    // A match arm is used here to display a custom message in the case of a 404 error.
    match resp.status().as_u16() {
        200 => {},
        404 => return Err("User does not exist.".into()),
        _ => {
            let message = format!("Unable to retrieve events. Status code: {}.", resp.status());
            return Err(message.into());
        }
    }

    let body = resp.text().await?;
    // Parsing response body, string -> vec of Rust structs
    let data: Vec<EventData> = serde_json::from_str(&body)?;  
    Ok(data)
}


pub async fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let username = parse_args(args)?;
    let data = make_request(&username).await?;   
    if data.is_empty() {
        println!("No events listed for this user.");
        return Ok(());
    }
    for event in data {
        println!("{event}");
    }
    Ok(())
}