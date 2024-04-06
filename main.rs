use anyhow::{Context, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let city = args.get(1);
    if args.len() != 2 || args.get(1).is_none() {
        println!("Usage: weather <city>");
        std::process::exit(0);
    }
    let city = city.unwrap().to_owned();
    println!("Fetching weather for {}", city);
    fetch_weather(city).await
}

async fn fetch_weather(city: String) -> Result<()> {
    use html2text::config;
    let url = format!("https://wttr.in/{}", city);
    let bytes = reqwest::get(&url)
        .await
        .context("Failed to make network request")?
        .bytes()
        .await
        .context("Failed to read into bytes")?;
    let mut reader = std::io::Cursor::new(bytes);
    let s = config::rich()
        .string_from_read(&mut reader, 150)
        .context("Render failed")?;
    println!("{s}");
    Ok(())
}
