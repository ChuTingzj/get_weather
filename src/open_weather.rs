use chalk_rs::Chalk;
use chrono::{DateTime, Local, Utc};
use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, UNIX_EPOCH};
pub const GET_CITY_HOST: &str = "https://api.openweathermap.org/geo/1.0/direct?";
pub const GET_WEATHER_HOST: &str = "https://api.openweathermap.org/data/2.5/weather?";
#[derive(Serialize, Deserialize, Debug)]
struct LocalNames {
    en: String,
    zh: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct OpenWeatherMain {
    feels_like: f32,
    humidity: f32,
    pressure: f32,
    temp: f32,
    temp_max: f32,
    temp_min: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct OpenWeatherSys {
    country: String,
    id: i32,
    sunrise: i32,
    sunset: i32,
}
#[derive(Serialize, Deserialize, Debug)]
struct OpenWeatherWeather {
    id: i32,
    description: String,
    icon: String,
    main: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct OpenWeatherWind {
    deg: f32,
    speed: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct OpenCity {
    country: String,
    lat: f32,
    lon: f32,
    name: String,
    local_names: LocalNames,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenWeather {
    main: OpenWeatherMain,
    sys: OpenWeatherSys,
    visibility: f32,
    weather: Box<[OpenWeatherWeather]>,
    wind: OpenWeatherWind,
}
#[derive(Debug)]
pub struct Weather {
    city: String,
}
impl Weather {
    pub fn new(city: String) -> Weather {
        Weather { city }
    }
    pub fn formate_timestamp(timestamp: i32) -> String {
        let time = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
        let datetime = DateTime::<Local>::from(time);
        datetime.format("%H:%M:%S").to_string()
    }
    pub async fn get_weather(&self) -> Result<OpenWeather, reqwest::Error> {
        let mut ch = Chalk::new();
        ch.bg_magenta()
            .cyan()
            .bold()
            .println(&format!("今天{}", Utc::now().format("%Y-%m-%d")));
        let mut chalk = Chalk::new();
        let city_query = format!(
            "q={}&limit=1&appid=e5c0d6f7175a8bbea898f97168780150",
            self.city
        );
        let res_city = reqwest::get([GET_CITY_HOST, city_query.as_str()].concat())
            .await?
            .json::<[OpenCity; 1]>()
            .await?;
        let lat = res_city[0].lat;
        let lon = res_city[0].lon;
        let city_name = &res_city[0].local_names.zh;
        let weather_query = format!(
            "lat={}&lon={}&units=metric&lang=zh_cn&appid=e5c0d6f7175a8bbea898f97168780150",
            lat, lon
        );
        let res_weather = reqwest::get([GET_WEATHER_HOST, weather_query.as_str()].concat())
            .await?
            .json::<OpenWeather>()
            .await?;
        chalk.magenta().bold().println(&format!(
            "🌄 {}当前温度:{}°C",
            city_name, res_weather.main.temp
        ));
        chalk.cyan().bold().println(&format!(
            "🧊 {}当前最低温度:{}°C",
            city_name, res_weather.main.temp_min
        ));
        chalk.red().bold().println(&format!(
            "🌋 {}当前最高温度:{}°C",
            city_name, res_weather.main.temp_max
        ));
        chalk.yellow().bold().println(&format!(
            "💪 {}当前体感温度:{}°C",
            city_name, res_weather.main.feels_like
        ));
        chalk.green().bold().println(&format!(
            "💧 {}当前湿度:{}%",
            city_name, res_weather.main.humidity
        ));
        chalk.green().bold().println(&format!(
            "🛬 {}当前风速:{}m/s",
            city_name, res_weather.wind.speed
        ));
        chalk.green().bold().println(&format!(
            "🌞 {}今日日出时间:{}",
            city_name,
            Weather::formate_timestamp(res_weather.sys.sunrise)
        ));
        chalk.green().bold().println(&format!(
            "🌜 {}今日日落时间:{}",
            city_name,
            Weather::formate_timestamp(res_weather.sys.sunset)
        ));
        chalk.blue().bold().println(&format!(
            "⛅ {}今日天气状况:{}",
            city_name, res_weather.weather[0].description
        ));
        Ok(res_weather)
    }
    pub fn value(&self) -> Option<&str> {
        Some(&self.city)
    }
}
