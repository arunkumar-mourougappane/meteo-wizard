use std::net::IpAddr;

use ipgeolocate::{Locator, Service};
use postcode::Postcode;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeoLocationError {
    #[error("Failed to parse IP.")]
    CannotParseIP,
    #[error("Failed to acquire location.")]
    CannotAcquireLocation(String),
    #[error("Cannot Parse location from postal code: {0}")]
    CannotParseFromPostCode(#[from] postcode::Error),
    #[error("Cannot obtain location using IP geolocate.")]
    GeolocationError(#[from] ipgeolocate::GeoError),
    #[error("Cannot parse `{0}` to `{1}.")]
    ParseError(String, String),
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeoLocationData {
    latitude: f64,
    longitude: f64,
    ip_address: String,
    postal_code: String,
    region: String,
}

impl GeoLocationData {
    /// Creates a new [`GeoLocationData`].
    pub fn new(
        latitude: f64,
        longitude: f64,
        ip_address: String,
        postal_code: String,
        region: String,
    ) -> GeoLocationData {
        Self {
            latitude,
            longitude,
            ip_address,
            postal_code,
            region,
        }
    }

    pub fn with_latitude(mut self, latitude: f64) -> GeoLocationData {
        self.latitude = latitude;
        self
    }

    pub fn with_longitude(mut self, longitude: f64) -> GeoLocationData {
        self.longitude = longitude;
        self
    }

    pub fn with_ip_address(mut self, ip_address: String) -> GeoLocationData {
        self.ip_address = ip_address;
        self
    }

    pub fn with_postal_code(mut self, postal_code: String) -> GeoLocationData {
        self.postal_code = postal_code;
        self
    }

    pub fn get_latitude(&self) -> f64 {
        self.latitude
    }

    pub fn get_longitude(&self) -> f64 {
        self.longitude
    }

    pub fn get_region(&self) -> String {
        self.region.clone()
    }
}

fn ip_addr_to_string(ip: IpAddr) -> String {
    match ip {
        IpAddr::V4(ipv4) => ipv4.to_string(),
        IpAddr::V6(ipv6) => ipv6.to_string(),
    }
}

pub async fn locate_from_postal_code(
    postal_code: String,
) -> Result<GeoLocationData, postcode::Error> {
    let postal_code_geo_data = Postcode::from_code(postal_code.clone()).await;
    let ip_address = match public_ip::addr().await {
        Some(ip) => ip_addr_to_string(ip),
        None => "".to_string(),
    };
    match postal_code_geo_data {
        Ok(postal_geo_data) => Ok(GeoLocationData::new(
            postal_geo_data.latitude,
            postal_geo_data.longitude,
            ip_address,
            postal_code,
            postal_geo_data.region,
        )),
        Err(error) => {
            log::error!("Cannot get location from postal code.");
            Err(error)
        }
    }
}

pub async fn local_from_public_ip() -> Result<GeoLocationData, GeoLocationError> {
    let ip_address = match public_ip::addr().await {
        Some(ip) => ip_addr_to_string(ip),
        None => return Err(GeoLocationError::CannotParseIP),
    };

    let service = Service::IpApi;

    match Locator::get(&ip_address, service).await {
        Ok(location_data) => {
            let latitude = match location_data.latitude.parse() {
                Ok(latitude) => latitude,
                Err(error) => {
                    log::error!("Cannot parse latitude to float value, error: {:?}", error);
                    return Err(GeoLocationError::ParseError(
                        location_data.latitude,
                        "f64".to_string(),
                    ));
                }
            };
            let longitude = match location_data.longitude.parse() {
                Ok(longitude) => longitude,
                Err(error) => {
                    log::error!("Cannot parse longitude to float value, error: {:?}", error);
                    return Err(GeoLocationError::ParseError(
                        location_data.longitude,
                        "f64".to_string(),
                    ));
                }
            };

            Ok(GeoLocationData::new(
                latitude,
                longitude,
                ip_address,
                "".to_string(),
                location_data.region,
            ))
        }
        Err(error) => {
            log::error!("Geolocation Parse error: {:?}", error);
            Err(GeoLocationError::GeolocationError(error))
        }
    }
}
