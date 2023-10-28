use dotenvy_macro::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use logging::setup_log;
use model::{DBParamsType, ScreepState};

mod logging;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log()?;
    dotenvy::dotenv()?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    log::info!("client init success");

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
    log::debug!("{:?}", state);

    let client: Client = client.clone();
    tokio::spawn(async move {
        if let Err(e) = save_to_db(state, client).await {
            log::error!("save to db error: {}", e);
        }
    });

    Ok(())
}

async fn save_to_db(state: ScreepState, client: Client) -> Result<(), Box<dyn std::error::Error>> {
    const URL: &str = "http://127.0.0.1:12000/api/v2/write";
    let resp = client
        .post(URL)
        .body(convert_state_to_params(state))
        .header(
            "Authorization",
            &("Token ".to_string() + dotenv!("DB_TOKEN")),
        )
        .query(&[("org", "boom"), ("bucket", "screeps"), ("precision", "s")])
        .send()
        .await?;
    let code = resp.status();
    if !code.is_success() {
        return Err(format!("save to db error: {}", code).into());
    }
    Ok(())
}

// better use macro
fn convert_state_to_params(state: ScreepState) -> String {
    let now = &std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();

    let mut params: Vec<DBParamsType> = vec![];

    {
        let p = DBParamsType::new()
            .set_time(now)
            .set_measurement("gcl")
            .add_field("level", state.gcl.level)
            .add_field("progress", state.gcl.progress)
            .add_field("progressTotal", state.gcl.progress_total);
        params.push(p);
    }

    {
        let p = DBParamsType::new()
            .set_time(now)
            .set_measurement("cpu")
            .add_field("bucket", state.cpu.bucket)
            .add_field("limit", state.cpu.limit)
            .add_field("used", state.cpu.used);
        params.push(p);
    }

    {
        for (room_name, v) in state.rooms {
            let p = DBParamsType::new()
                .set_time(now)
                .set_measurement("room")
                .add_tag("room_name", room_name)
                .add_field("storageEnergy", v.storage_energy)
                .add_field("terminalEnergy", v.terminal_energy)
                .add_field("energyAvailable", v.energy_available)
                .add_field("energyCapacityAvailable", v.energy_capacity_available)
                .add_field("controllerProgress", v.controller_progress)
                .add_field("controllerProgressTotal", v.controller_progress_total)
                .add_field("controllerLevel", v.controller_level);
            params.push(p);
        }
    }

    {
        let p = DBParamsType::new()
            .set_time(now)
            .set_measurement("extra")
            .add_field("tick", state.time);
        params.push(p);
    }

    params
        .into_iter()
        .map(|x| x.build())
        .collect::<Vec<_>>()
        .join("\n")
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

    #[test]
    fn test_convert_state_to_params() {
        let input = r#"{"gcl":{"progress":2184854.0,"progressTotal":4278031.643091577,"level":2},"cpu":{"bucket":10000,"limit":20,"used":6.357330299913883},"time":52538463,"rooms":{"W21N38":{"storageEnergy":0,"terminalEnergy":0,"energyAvailable":26,"energyCapacityAvailable":550,"controllerProgress":122194,"controllerProgressTotal":135000,"controllerLevel":3,"creepRoleAmount":{}}}}"#;

        let state = serde_json::from_str::<ScreepState>(input).expect("unmarshal error");
        let res = super::convert_state_to_params(state);
        println!("{}", res);
    }
}
