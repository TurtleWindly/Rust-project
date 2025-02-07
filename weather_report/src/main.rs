use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
    sea_level: Option<u32>,
    grnd_level: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Clouds {
    all: u32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Sys {
    r#type: Option<u32>,
    id: Option<u32>,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = dotenvy::dotenv() {
        eprintln!("Error loading .env file: {}", e);
        std::process::exit(1);
    }

    let api_key = std::env::var("API_KEY").unwrap();

    // Main program
    let mut lat = String::new();
    let mut lon = String::new();
    println!("Enter lat: ");
    std::io::stdin().read_line(&mut lat).unwrap();
    println!("Enter lon: ");
    std::io::stdin().read_line(&mut lon).unwrap();

    // HCM lat and lon
    // let lat = "10.776530";
    // let lon = "106.700981";

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        lat, lon, api_key
    );
    let resp = reqwest::get(url).await?.json::<WeatherResponse>().await?;
    println!("{resp:#?}");
    Ok(())
}
