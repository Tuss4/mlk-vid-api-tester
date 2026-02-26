use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RandomMLKVid {
    video_id: String,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://mlk.tjsoptame.dev/videos/random")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();
    println!("{response:?}");

    assert!(response.status() == reqwest::StatusCode::OK);

    let response_data = response.json::<RandomMLKVid>().await;

    println!("{response_data:?}");

    assert_ne!(response_data.unwrap().video_id, "");
}
