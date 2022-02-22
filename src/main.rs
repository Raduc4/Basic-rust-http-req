use std::collections::HashMap;
use dotenv::dotenv;
use std::env;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct Post {
    id: i32,
    title: String,
    body: String,
    userId: i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("BREWS_API_KEY").unwrap();
    let api_token = env::var("BREWS_API_TOKEN").unwrap() ;

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", api_token).parse().unwrap());
    headers.insert("apikey", api_key.parse().unwrap());

    // let mut post_to_send = HashMap::new();
    let client = reqwest::Client::new();
    let p =  Post
    { id: 23131312,
        title: String::from("XD"),
        body:  String::from("Something"),
        userId: 131
    };
    let resp_json = client
        .post("https://jsonplaceholder.typicode.com/posts")
        // .get("url").json::<Vec<Post>>()
        .json::<Post>(&p)
        .send()
        .await?;

    let resp_json = resp_json.json::<Post>().await?;
    println!("{:#?}", resp_json);
    Ok(())
}