use reqwest::header::{AUTHORIZATION, USER_AGENT};

struct CatFact {
    fact: String,
    length: i32,
}

#[tokio::main]
async fn main() {
    let subs = get_subs().await;

    println!("subs = {:#?}", subs);
}

async fn get_subs() -> Result<String, Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let body = client.get("https://oauth.reddit.com/subreddits/mine/subscriber")
        .header(USER_AGENT, "foo")
        .header(AUTHORIZATION, format!("bearer {}", env!("REDDIT")))
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
