use std::env;

#[tokio::main]
async fn main() {
    let subs = get_subs().await;

    println!("subs = {:#?}", subs);
}

async fn get_subs() -> Result<String, Box<dyn std::error::Error>> {

    use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

    let _token = env::var("REDDIT");
    let client = reqwest::Client::new();
    let body = client.get("https://oauth.reddit.com/subreddits/mine/subscriber")
        .header(CONTENT_TYPE, "application/json")
        .header(USER_AGENT, "foo")
        .header(AUTHORIZATION, "bearer {_token}")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
