use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env} ;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    local_names: Option<HashMap<String, String>>,
    lat: f64,
    lon: f64,
    country: String,
    state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LatLon {
    pub lat: f64,
    pub lon: f64,
}

type Locations = Vec<Location>;

pub async fn get_weather(city: &str) -> Result<Option<WeatherResult>, Box<dyn std::error::Error>> {
    let lat_lon = get_lat_lon(city).await?;

    let (lat, lon) = match lat_lon {
        Some(lat_lon) => (lat_lon.lat, lat_lon.lon),
        None => {
            return Ok(None);
        }
    };

    let weather = get_lat_lon_weather(lat, lon).await?;

    Ok(Some(weather))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub sea_level: Option<f32>,
    pub grnd_level: Option<f32>,
    pub humidity: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    pub id: i64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: f32,
    pub gust: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResult {
    pub coord: LatLon,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: f64,
    pub wind: Wind,
    pub rain: Option<HashMap<String, f32>>,
    pub snow: Option<HashMap<String, f32>>,
    pub clouds: Option<HashMap<String, f32>>,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i64,
}

async fn get_lat_lon_weather(
    lat: f64,
    lon: f64,
) -> Result<WeatherResult, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric&appid={}",
        lat, lon, "6587789bbac1177faff8c858222f56f6"
    ))
    .await?
    .json::<WeatherResult>()
    .await?;

    Ok(response)
}

async fn get_lat_lon(city: &str) -> Result<Option<LatLon>, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(
        "http://api.openweathermap.org/geo/1.0/direct?limit=1&q={}&appid={}",
        city, "6587789bbac1177faff8c858222f56f6"
    ))
    .await?
    .json::<Locations>()
    .await?;

    if !response.is_empty() {
        let city = &response[0];
        let lat_lon = LatLon {
            lat: city.lat,
            lon: city.lon,
        };
        Ok(Some(lat_lon))
    } else {
        Ok(None)
    }
}
