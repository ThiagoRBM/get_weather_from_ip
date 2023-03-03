//https://levelup.gitconnected.com/rust-making-http-requests-and-handling-responses-by-using-reqwest-c8d557cdce46
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GETIPinfo {
    ip: String,
    continent: String,
    continent_code: String,
    country: String,
    region: String,
    region_code: String,
    latitude: f64,
    longitude: f64,
    timezone: TimeZone,
} // não precisa colocar todos os elementos do JSON da API no struct

#[derive(Serialize, Deserialize, Debug)]
struct TimeZone {
    current_time: String,
    utc: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GETAPIResponse {
    origin: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CoordenadasIp {
	ip: String,
    latitude: f64,
    longitude: f64,
    region: String,
    utc: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherInfo {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Serialize, Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    weathercode: f64,
    time: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InfoTotal {
	ip: String,
	region: String,
	longitude: f64,
	latitude: f64,
	temperature: f64,
	windspeed: f64,
	time: String,
    utc: String,
}

#[tokio::main]
async fn get_public_ip() -> Result<std::string::String, Box<dyn std::error::Error>> {
    // - Create a new client which is re-used between requests
    let client = reqwest::Client::new();

    let ip = client
        .get("https://httpbin.org/ip")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<GETAPIResponse>()
        .await?;

    // println!("\n\nIP busca: {}\n\n", ip.origin);
    // println!("{}", ip);
    Ok(ip.origin)
}

#[tokio::main]
async fn get_ip_info(ip: &str) -> Result<CoordenadasIp, Box<dyn std::error::Error>> {
    let url = String::from("https://ipwho.is/") + &ip; // concatenando https://maxuuell.com/blog/how-to-concatenate-strings-in-rust
    // println!("\niniciando\n");

    // - Create a new client which is re-used between requests
    let client = reqwest::Client::new();

    let resp200 = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        // .text();
        .json::<GETIPinfo>()
        .await?;

    // println!("{:#?}\n\n\n", resp200);
    // println!("IP: {}\nCidade: {}\nLongitude: {} Latitude: {}\nTime_zone: {}", resp200.ip, resp200.region, resp200.longitude, resp200.latitude, resp200.timezone.current_time);

    let coords = CoordenadasIp {
    	ip: resp200.ip,
        latitude: resp200.latitude,
        longitude: resp200.longitude,
        region: resp200.region,
        utc: resp200.timezone.utc
    };

    // println!("{:#?}", coords);

    Ok(coords)
}

// funcao para a partir das coordenadas, pegar as informacoes pela API, usando esse formato: https://api.open-meteo.com/v1/forecast?latitude=-15.7942287&longitude=-47.8821658&current_weather=true
#[tokio::main]
async fn get_current_weather(
    coordenadas: &CoordenadasIp,
) -> Result<WeatherInfo, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        coordenadas.latitude, coordenadas.longitude
    );
	// println!("\n\n\niniciando\n\n\n");
    let client = reqwest::Client::new();

    let weather = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        // .text();
        .json::<WeatherInfo>()
        .await?;

	// println!("{:#?}", weather);

    Ok(weather)
}

//https://stackoverflow.com/questions/71803908/how-to-make-function-that-returns-text-from-response
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ip = get_public_ip()?;
    // println!("IP:\n{:#?}\n", ip);

    let _ip_info = get_ip_info(&ip)?;
    // println!("Coordenadas:\n{:#?}\n", _ip_info);

    let c_weather = get_current_weather(&_ip_info)?;
    // println!("Current Weather:\n	temp °C: {:#?}\n	windspeed m/s: {:#?}\n", c_weather.current_weather.temperature,c_weather.current_weather.windspeed);

    let info = InfoTotal{
        ip: _ip_info.ip,
        region: _ip_info.region,
        longitude: _ip_info.longitude,
        latitude: _ip_info.latitude,
        temperature: c_weather.current_weather.temperature,
        windspeed: c_weather.current_weather.windspeed,
        time: c_weather.current_weather.time,
        utc: _ip_info.utc
    };

    // println!("{:#?}", info);
    println!("ip: {}\nlat: {}\nlong: {}\ntemp: {}\nwindspeed: {}", info.ip, info.longitude, info.latitude, info.temperature, info.windspeed);

    Ok(())
}
