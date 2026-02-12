mod app;
mod awdio;
mod config;
mod ignite;
mod result;
mod ui;
mod logger;

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
