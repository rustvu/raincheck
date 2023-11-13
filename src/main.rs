//! Raincheck - example client application for web services in Rust

use std::collections::HashMap;

use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Location {
    lat: f64,
    lon: f64,
}

/// Get my IP address using ipify API
/// See documentation at: https://www.ipify.org/
fn get_myip(client: &Client) -> String {
    #[derive(Deserialize, Debug)]
    struct Ip {
        ip: String,
    }

    let reponse = client
        .get("https://api.ipify.org")
        .query(&[("format", "json")])
        .send()
        .unwrap();
    // println!("{}", reponse.text().unwrap());

    let ip: Ip = reponse.json().unwrap();
    ip.ip
}

/// Get location information for an IP address IP Geolocation API
/// See documentation at: https://ip-api.com/docs
fn get_location(client: &Client, ip: &str) -> Location {
    let reponse = client
        .get(format!("http://ip-api.com/json/{ip}"))
        .send()
        .unwrap();
    // println!("{}", reponse.text().unwrap());

    let location: Location = reponse.json().unwrap();
    location
}

/// Get rain forecast for a location using NWS API
/// See documentation at: https://www.weather.gov/documentation/services-web-api
fn get_rain_forecast(client: &Client, location: &Location) -> Vec<(String, u64)> {
    #[derive(Deserialize, Debug)]
    struct GeneralResponse {
        properties: HashMap<String, serde_json::Value>,
    }

    let reponse = client
        .get(format!(
            "https://api.weather.gov/points/{},{}",
            location.lat, location.lon
        ))
        .send()
        .unwrap();
    // println!("{}", reponse.text().unwrap());

    let general_response: GeneralResponse = reponse.json().unwrap();
    // println!("{:#?}", general_response);

    let forecast_url = general_response
        .properties
        .get("forecast")
        .unwrap()
        .as_str()
        .unwrap();
    // println!("{}", forecast_url);

    let reponse = client.get(forecast_url).send().unwrap();

    let general_response: GeneralResponse = reponse.json().unwrap();
    // println!("{:#?}", general_response);

    let periods = general_response
        .properties
        .get("periods")
        .unwrap()
        .as_array()
        .unwrap();
    //println!("{:#?}", periods);

    let mut forecast = Vec::new();
    for period in periods {
        let name = period.get("name").unwrap().as_str().unwrap().to_string();
        let chance_of_rain = period
            .get("probabilityOfPrecipitation")
            .unwrap()
            .as_object()
            .unwrap()
            .get("value")
            .unwrap()
            .as_u64()
            .unwrap_or(0);
        forecast.push((name.to_string(), chance_of_rain));
    }
    forecast
}

fn main() {
    let client = Client::builder()
        .user_agent("Raincheck / Vanderbilt CS 3891")
        .build()
        .unwrap();

    let my_ip = get_myip(&client);
    println!("My IP: {}", my_ip);

    let my_location = get_location(&client, &my_ip);
    println!("My location: {:?}", my_location);

    let forecast = get_rain_forecast(&client, &my_location);
    for (time, forecast) in forecast {
        println!("{:16}: {:2} %", time, forecast);
    }
}
