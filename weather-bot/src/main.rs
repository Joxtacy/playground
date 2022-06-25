use std::env;

use weather_bot::get_weather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        panic!("Too few arguments");
    }

    let city = &args[1];

    let weather = get_weather(city).await?;

    println!("DEBUG: {:?}", weather);
    match weather {
        Some(weather) => {
            println!(
                "Current weather in {}, {}: {}, {}°C with a humidity of {}%.",
                weather.name,
                weather.sys.country,
                weather.weather[0].description,
                weather.main.temp,
                weather.main.humidity
            );
        }
        None => {
            println!("No weather for you pal");
        }
    }

    Ok(())
}
