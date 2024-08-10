use core::fmt;
use std::str::FromStr;

const OPEN_METEO_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEnumError;

#[derive(Debug)]
pub enum HourlyTempFromGround {
    Unspecified,
    TempAt2m,
    TempAt80m,
    TemAt120m,
    TemAt180m,
}

impl fmt::Display for HourlyTempFromGround {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HourlyTempFromGround::Unspecified => write!(f, ""),
            HourlyTempFromGround::TempAt2m => write!(f, "hourly=temperature_2m"),
            HourlyTempFromGround::TempAt80m => write!(f, "hourly=temperature_80m"),
            HourlyTempFromGround::TemAt120m => write!(f, "hourly=temperature_120m"),
            HourlyTempFromGround::TemAt180m => write!(f, "hourly=temperature_180m"),
        }
    }
}

impl FromStr for HourlyTempFromGround {
    fn from_str(temperature_set: &str) -> Result<HourlyTempFromGround, ParseEnumError> {
        match temperature_set {
            "1" => Ok(Self::TempAt2m),
            "2" => Ok(Self::TempAt80m),
            "3" => Ok(Self::TemAt120m),
            "4" => Ok(Self::TemAt180m),
            _ => Ok(Self::Unspecified),
        }
    }

    type Err = ParseEnumError;
}

#[derive(Debug)]
pub struct UrlConfig {
    latitude: f64,
    longitude: f64,
    hourly_temp_gnd_level: HourlyTempFromGround,
    relative_humidity_2m: bool,
    apparent_temperature: bool,
    precipitation_probability: bool,
    precipitation: bool,
    rain: bool,
    showers: bool,
    snowfall: bool,
    weather_code: bool,
    visibility: bool,
    forecast_days: u32,
    past_days: u32,
}

impl UrlConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        latitude: f64,
        longitude: f64,
        hourly_temp_gnd_level: HourlyTempFromGround,
        relative_humidity_2m: bool,
        apparent_temperature: bool,
        precipitation_probability: bool,
        precipitation: bool,
        rain: bool,
        showers: bool,
        snowfall: bool,
        weather_code: bool,
        visibility: bool,
        forecast_days: u32,
        past_days: u32,
    ) -> UrlConfig {
        UrlConfig {
            latitude,
            longitude,
            hourly_temp_gnd_level,
            relative_humidity_2m,
            apparent_temperature,
            precipitation_probability,
            precipitation,
            rain,
            showers,
            snowfall,
            weather_code,
            visibility,
            forecast_days,
            past_days,
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub fn build(
        latitude: f64,
        longitude: f64,
        hourly_temp_gnd_level: HourlyTempFromGround,
        relative_humidity_2m: bool,
        apparent_temperature: bool,
        precipitation_probability: bool,
        precipitation: bool,
        rain: bool,
        showers: bool,
        snowfall: bool,
        weather_code: bool,
        visibility: bool,
        forecast_days: u32,
        past_days: u32,
    ) -> UrlConfig {
        UrlConfig {
            latitude,
            longitude,
            hourly_temp_gnd_level,
            relative_humidity_2m,
            apparent_temperature,
            precipitation_probability,
            precipitation,
            rain,
            showers,
            snowfall,
            weather_code,
            visibility,
            forecast_days,
            past_days,
        }
    }

    pub fn with_latitude(mut self, latitude: f64) -> Self {
        self.latitude = latitude;
        self
    }

    pub fn with_longitude(mut self, longitude: f64) -> Self {
        self.longitude = longitude;
        self
    }

    pub fn with_forecast_days(mut self, forecast_days: u32) -> Self {
        self.forecast_days = forecast_days;
        self
    }

    pub fn with_past_days(mut self, past_days: u32) -> Self {
        self.past_days = past_days;
        self
    }
}

impl fmt::Display for UrlConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}?latitude={:.3}&longitude&longitude={:.3}&{}{}{}{}{}{}{}{}{}{}{}{}",
            OPEN_METEO_API_URL,
            self.latitude,
            self.longitude,
            self.hourly_temp_gnd_level,
            if self.relative_humidity_2m {
                ",relative_humidity_2m"
            } else {
                ""
            },
            if self.apparent_temperature {
                ",apparent_temperature"
            } else {
                ""
            },
            if self.precipitation_probability {
                ",precipitation_probability"
            } else {
                ""
            },
            if self.precipitation {
                ",precipitation"
            } else {
                ""
            },
            if self.rain { ",rain" } else { "" },
            if self.showers { ",showers" } else { "" },
            if self.snowfall { ",snowfall" } else { "" },
            if self.weather_code {
                ",weather_code"
            } else {
                ""
            },
            if self.visibility { ",visibility" } else { "" },
            if self.forecast_days != 0 {
                format!("&forecast_days={}", self.forecast_days)
            } else {
                "".to_string()
            },
            if self.past_days != 0 {
                format!("&past_days={}", self.past_days)
            } else {
                "".to_string()
            },
        )
    }
}
