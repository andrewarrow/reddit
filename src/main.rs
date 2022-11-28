use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct SubList {
    data: SubData
}

#[derive(Deserialize, Debug)]
struct SubData {
    children: Vec<Sub>
}

#[derive(Deserialize, Debug)]
struct Sub {
    //kind: String,
    data: SubDetail
}

#[derive(Deserialize, Debug)]
struct SubDetail {
    display_name: String
}

#[tokio::main]
async fn main() {
    let subs = match get_subs().await {
        Ok(v) => v,
        Err(e) => { 
            println!("e {}", e); 
            SubList{data: SubData{children: Vec::new()}}
        }
    };

    for sub in subs.data.children.iter() {
      println!("subs = {:#?}", sub.data.display_name);
    }

}

async fn get_subs() -> Result<SubList, Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let body = client.get("https://oauth.reddit.com/subreddits/mine/subscriber")
        .header(USER_AGENT, "foo")
        .header(AUTHORIZATION, format!("bearer {}", env!("REDDIT")))
        .send()
        .await?
        .json::<SubList>()
        .await?;

    Ok(body)
}
