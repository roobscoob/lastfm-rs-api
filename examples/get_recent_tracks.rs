use std::env;

use chrono::{DateTime, NaiveDate, Utc};
use futures::StreamExt;
use lastfm_rs_api::{LastFm, authentication::public::PublicAuthentication};

const START: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
    NaiveDate::from_ymd_opt(2024, 8, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap(),
    Utc,
);

const END: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
    NaiveDate::from_ymd_opt(2024, 8, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap(),
    Utc,
);

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let _ = dotenvy::from_filename(".env.local");

    let authentication = PublicAuthentication::new(env::var("API_KEY").unwrap().as_str());
    let mut client = LastFm::new().with_authentication(authentication);

    client
        .user_get_recent_tracks("roobscoob")
        .with_start_date(START)
        .with_end_date(END)
        .fetch()
        .await
        .unwrap()
        .take(100)
        .for_each(async |v| println!("{:?}", v))
        .await;
}
