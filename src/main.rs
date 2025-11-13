mod app;
mod event;
mod tui;
mod ui;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tui::run().await?;
    Ok(())
}