use std::env;

use futures::StreamExt;
use lastfm_rs_api::{LastFm, authentication::public::PublicAuthentication};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename(".env.local");

    let authentication = PublicAuthentication::new(env::var("API_KEY").unwrap().as_str());
    let mut client = LastFm::new().with_authentication(authentication);

    client
        .user_get_recent_tracks("roobscoob")
        .await
        .fetch()
        .await
        .unwrap()
        .take(100)
        .for_each(async |v| println!("{:?}", v))
        .await;
}
