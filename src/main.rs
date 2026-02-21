mod app;
mod awdio;
mod config;
mod ignite;
mod logger;
mod result;
mod ui;

#[tokio::main]
async fn main() -> result::EchoResult<()> {
    logger::init_logger();

    match ignite::engine() {
        Ok(val) => {
            if let Err(e) = app::start(val).await {
                eprintln!("{}", e);
            }
        }
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}
