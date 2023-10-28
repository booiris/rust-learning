pub fn setup_log() -> Result<(), Box<dyn std::error::Error>> {
    if !std::path::Path::new("log/").exists() {
        std::fs::create_dir("log")?;
    }

    let log = fern::Dispatch::new()
        .level(log::LevelFilter::Info)
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
        .chain(fern::log_file("./log/output.log")?);

    #[cfg(debug_assertions)]
    let log = log.level(log::LevelFilter::Debug);

    log.apply()?;
    Ok(())
}

#[cfg(test)]
pub fn setup_log_test() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Trace)
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
        .apply()?;
    Ok(())
}
