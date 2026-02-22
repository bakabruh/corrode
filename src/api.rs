use serde::Deserialize;

const API_BASE: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Deserialize, Clone, Debug)]
pub struct WeatherResponse {
    pub current_weather: CurrentWeather
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub windspeed: f64,
}

pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!("{}?latitude={}&longitude={}&current_weather=true", API_BASE, lat, lon);
    reqwest::get(url).await?.json::<WeatherResponse>().await
}

