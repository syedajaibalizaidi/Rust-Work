// Network IO , two creates i rust reqwest and hyper which is super powerful than reqwest 
// crate needed -> serde, tokio, anyhow for errors 

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64, 
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,  
    windspeed: f64,   
    winddirection: f64,
    weathercode: i32,
    time: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current_weather=true";
    let response = reqwest::get(URL).await?;
    let weather: Weather = response.json().await?;  
    println!("{:#?}", weather);
    Ok(())
}
