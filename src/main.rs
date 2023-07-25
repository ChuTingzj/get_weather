use chalk_rs::Chalk;
use std::io;
mod open_weather;
#[tokio::main]
async fn main() {
    let mut chalk = Chalk::new();
    let mut city = String::new();
    chalk.green().bold().println(&"请输入你的城市:");
    io::stdin().read_line(&mut city).unwrap();
    let ow = open_weather::Weather::new(city.trim().to_string());
    ow.get_weather().await.expect("err");
}
