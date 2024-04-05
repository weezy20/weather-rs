use anyhow::{Context, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let city = args.get(1);
    if args.len() != 2 || args.get(1).is_none() {
        println!("Usage: weather <city>");
        std::process::exit(0);
    }
    let city = city.unwrap().to_owned();
    println!("Fetching weather for {}", city);
    fetch_weather(city)
}

fn fetch_weather(city: String) -> Result<()> {
    let url = format!("https://wttr.in/{}", city);
    std::process::Command::new("curl")
        .arg(url)
        .spawn()
        .context("Failed to fetch weather command")?;
    Ok(())
}
