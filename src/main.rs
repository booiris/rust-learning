use dotenvy_macro::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use logging::setup_log;
use model::ScreepState;

mod logging;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log()?;
    dotenvy::dotenv()?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
        if let Err(e) = handle_screep_resp(&client).await {
            log::error!("error: {}", e);
        }
    }
}

async fn handle_screep_resp(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct ScreepResponse {
        ok: i64,
        data: String,
    }

    const URL: &str = "https://screeps.com/api/user/memory-segment";
    let resp = client
        .get(URL)
        .header("X-Token", dotenv!("SCREEP_TOKEN"))
        .query(&[("segment", "0"), ("shard", "shard3")])
        .send()
        .await?;
    let code = resp.status();
    let resp = resp
        .json::<ScreepResponse>()
        .await
        .map_err(|e| format!("unmarshal error: {}, code: {}", e, code))?;
    let state = serde_json::from_str::<ScreepState>(&resp.data)?;
    log::info!("{:?}", state);

    let client: Client = client.clone();
    tokio::spawn(async move {
        if let Err(e) = save_to_db(state, client).await {
            log::error!("save to db error: {}", e);
        }
    });

    Ok(())
}

async fn save_to_db(_state: ScreepState, client: Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "http://127.0.0.1:12000/api/v2/write";
    const TOKEN: &str = dotenv!("DB_TOKEN");
    let resp = client
        .post(URL)
        .header("Authorization", &("Token ".to_string() + TOKEN))
        .query(&[("org", "boom"), ("bucket", "screeps"), ("precision", "s")])
        .send()
        .await?;
    let code = resp.status();
    if !code.is_success() {
        return Err(format!("save to db error: {}", code).into());
    }
    Ok(())
}

#[cfg(test)]
mod unit {
    use std::time::Duration;

    use crate::{logging::setup_log_test, model::ScreepState};

    fn get_client() -> reqwest::Client {
        setup_log_test().expect("setup log error");
        reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("build client error")
    }

    #[tokio::test]
    async fn test_save_to_db() {
        let client = get_client();
        let state = ScreepState::default();
        if let Err(e) = super::save_to_db(state, client).await {
            panic!("save to db error: {}", e);
        }
    }
}
