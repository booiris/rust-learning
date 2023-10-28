use std::{thread, time::Duration};

const SCREEPS_URL: &str = "https://screeps.com/api/user/memory-segment";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log()?;

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    loop {
        let resp = client
            .get(SCREEPS_URL)
            .header("X-Token", "97455b9b-335b-4c50-96af-af1a0fe42262")
            .query(&[("segment", "0"), ("shard", "shard3")])
            .send();
        log::info!("resp: {:?}", resp);
        log::info!("resp: {:?}", resp?.text());
        thread::sleep(Duration::from_secs(5));
    }
}

fn setup_log() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
