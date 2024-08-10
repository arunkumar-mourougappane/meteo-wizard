use meteo_wizard::{
    geo_location::{self},
    settings::url_config::{HourlyTempFromGround, UrlConfig},
    weather_data::weather_point::WeatherData,
    web_protocols::http_fetch,
};

use std::process::exit;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let geo_location_data = match geo_location::geo_data::local_from_public_ip().await {
        Ok(location_data) => location_data,
        Err(error) => {
            log::error!("{}", error);
            exit(-1)
        }
    };

    let url_config = UrlConfig::new(
        geo_location_data.get_latitude(),
        geo_location_data.get_longitude(),
        HourlyTempFromGround::TempAt2m,
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        true,
        2,
        2,
    );

    let weather_data_str = match http_fetch::perform_http_get(url_config.to_string()) {
        Ok(weather_data_string) => weather_data_string,
        Err(error) => {
            log::error!("Failed to fetch weather data: {:?}", error);
            "".to_string()
        }
    };

    if weather_data_str.is_empty() {
        log::error!("Failed to fetch any data.");
        exit(-1)
    }

    let weather_json: serde_json::Value = match serde_json::from_str(&weather_data_str) {
        Ok(weather_json) => weather_json,
        Err(_) => {
            log::error!("cannot parse json data");
            exit(-3);
        }
    };

    let weather_data = WeatherData::parse_from(weather_json);
    match weather_data {
        Ok(weather_data) => {
            log::debug!("\n{}\n", weather_data);
            log::debug!("{}", weather_data.hourly_units);
            let data_points = weather_data.hourly;

            let mut sorted_time: Vec<&i64> = data_points.keys().collect();
            sorted_time.sort();
            for timestamp in sorted_time {
                log::debug!(
                    "{}",
                    match data_points.get(timestamp) {
                        Some(data_point) => data_point,
                        None => exit(-3),
                    }
                )
            }
        }
        Err(error) => log::error!("{}", error),
    }
}
