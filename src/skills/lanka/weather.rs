//! Sri Lanka weather data from the Department of Meteorology.
//!
//! Source: <https://www.meteo.gov.lk>
//!
//! Provides:
//! - Current weather conditions by district
//! - Monsoon status and alerts
//! - UV index and marine forecasts

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const METEO_BASE_URL: &str = "https://www.meteo.gov.lk";
const METEO_FORECAST_API: &str = "https://www.meteo.gov.lk/api/forecast";

/// Sri Lanka districts for weather queries.
pub const DISTRICTS: &[&str] = &[
    "Colombo", "Gampaha", "Kalutara", "Kandy", "Matale", "Nuwara Eliya",
    "Galle", "Matara", "Hambantota", "Jaffna", "Kilinochchi", "Mannar",
    "Vavuniya", "Mullaitivu", "Batticaloa", "Ampara", "Trincomalee",
    "Kurunegala", "Puttalam", "Anuradhapura", "Polonnaruwa", "Badulla",
    "Monaragala", "Ratnapura", "Kegalle",
];

/// Active monsoon seasons in Sri Lanka.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonsoonSeason {
    /// South-West monsoon: May–September (affects western/southern coast)
    SouthWest,
    /// North-East monsoon: November–February (affects eastern/northern coast)
    NorthEast,
    /// Inter-monsoon: transitional periods (April, October)
    InterMonsoon,
    /// Dry season
    Dry,
}

/// Weather forecast for a Sri Lankan district.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistrictForecast {
    pub district: String,
    pub condition: String,
    pub temp_min_c: f64,
    pub temp_max_c: f64,
    pub humidity_pct: u8,
    pub rainfall_mm: f64,
    pub uv_index: u8,
    pub wind_kmh: f64,
    pub monsoon_alert: Option<String>,
    /// Sinhala condition label
    pub condition_si: String,
    /// Tamil condition label
    pub condition_ta: String,
}

/// Marine forecast for coastal tourism.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarineForecast {
    pub area: String,
    pub wave_height_m: f64,
    pub wind_speed_kmh: f64,
    pub visibility_km: f64,
    pub safe_for_swimming: bool,
    pub advisory: Option<String>,
}

/// Fetch weather forecast for a given Sri Lanka district.
///
/// Falls back to the public page URL if the JSON API is unavailable.
pub async fn fetch_district_forecast(
    client: &reqwest::Client,
    district: &str,
) -> Result<DistrictForecast> {
    let url = format!("{METEO_FORECAST_API}?district={}", urlencoding::encode(district));

    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/thaaaru/apeclaw-labs)")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            r.json::<DistrictForecast>()
                .await
                .context("Failed to parse district forecast JSON")
        }
        _ => {
            // Return a stub pointing to the official page for agent web_fetch fallback
            Ok(DistrictForecast {
                district: district.to_string(),
                condition: format!("See {METEO_BASE_URL} for current forecast"),
                temp_min_c: 0.0,
                temp_max_c: 0.0,
                humidity_pct: 0,
                rainfall_mm: 0.0,
                uv_index: 0,
                wind_kmh: 0.0,
                monsoon_alert: None,
                condition_si: "දත්ත නොමැත".to_string(),
                condition_ta: "தரவு இல்லை".to_string(),
            })
        }
    }
}

/// Determine the current monsoon season based on month (1–12).
pub fn current_monsoon_season(month: u8) -> MonsoonSeason {
    match month {
        5..=9 => MonsoonSeason::SouthWest,
        11..=12 | 1..=2 => MonsoonSeason::NorthEast,
        4 | 10 => MonsoonSeason::InterMonsoon,
        _ => MonsoonSeason::Dry,
    }
}

/// Return a tourist-friendly monsoon advisory for the given month.
pub fn monsoon_advisory(month: u8) -> &'static str {
    match current_monsoon_season(month) {
        MonsoonSeason::SouthWest => {
            "South-West monsoon active (May–Sep). Western and southern coasts experience heavy rain. \
             East coast (Trincomalee, Arugam Bay) is ideal now."
        }
        MonsoonSeason::NorthEast => {
            "North-East monsoon active (Nov–Feb). Eastern and northern regions experience rain. \
             West coast (Colombo, Galle, Mirissa) is ideal now."
        }
        MonsoonSeason::InterMonsoon => {
            "Inter-monsoon period. Thunderstorms possible island-wide. \
             Check district forecasts before travel."
        }
        MonsoonSeason::Dry => {
            "Dry season. Generally good conditions island-wide for tourism."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monsoon_season_june_is_southwest() {
        assert!(matches!(current_monsoon_season(6), MonsoonSeason::SouthWest));
    }

    #[test]
    fn monsoon_season_december_is_northeast() {
        assert!(matches!(current_monsoon_season(12), MonsoonSeason::NorthEast));
    }

    #[test]
    fn monsoon_advisory_is_non_empty() {
        for month in 1u8..=12 {
            assert!(!monsoon_advisory(month).is_empty());
        }
    }

    #[test]
    fn all_districts_listed() {
        assert!(DISTRICTS.contains(&"Colombo"));
        assert!(DISTRICTS.contains(&"Jaffna"));
        assert!(DISTRICTS.contains(&"Galle"));
        assert_eq!(DISTRICTS.len(), 25);
    }

    #[test]
    fn sinhala_tamil_stub_strings_valid_utf8() {
        let si = "දත්ත නොමැත";
        let ta = "தரவு இல்லை";
        assert!(si.chars().count() > 0);
        assert!(ta.chars().count() > 0);
    }
}
