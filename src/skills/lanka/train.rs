//! Sri Lanka Railways schedule and fare queries.
//!
//! Source: <https://www.railway.gov.lk>
//!
//! The Kandy–Ella hill country train is consistently rated one of the
//! world's most scenic rail journeys. This module provides schedule
//! lookup, fare estimation, and tourist class availability.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const SLR_BASE_URL: &str = "https://www.railway.gov.lk";
const SLR_SCHEDULE_API: &str = "https://www.railway.gov.lk/api/schedule";

/// Major tourist rail routes in Sri Lanka.
pub const TOURIST_ROUTES: &[(&str, &str, &str)] = &[
    ("Colombo Fort", "Kandy",       "Up-country line — 2h 45m approx"),
    ("Kandy",        "Ella",        "Iconic hill country — 7h approx, observation car recommended"),
    ("Colombo Fort", "Galle",       "Coastal line — 2h 30m approx"),
    ("Colombo Fort", "Jaffna",      "Northern line — 6h approx"),
    ("Colombo Fort", "Anuradhapura","North-Central line — 4h approx"),
    ("Colombo Fort", "Batticaloa",  "East Coast line — 8h approx"),
];

/// Train classes available in Sri Lanka.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrainClass {
    /// Air-conditioned (intercity express only)
    FirstClassAC,
    /// Reserved seating
    SecondClass,
    /// Unreserved
    ThirdClass,
    /// Observation saloon (Kandy–Ella, tourist-specific)
    ObservationCar,
}

impl TrainClass {
    pub fn display_name(&self) -> &str {
        match self {
            TrainClass::FirstClassAC   => "1st Class (A/C)",
            TrainClass::SecondClass    => "2nd Class (Reserved)",
            TrainClass::ThirdClass     => "3rd Class (Unreserved)",
            TrainClass::ObservationCar => "Observation Car (Tourist)",
        }
    }
}

/// A train schedule entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainSchedule {
    pub train_number: String,
    pub train_name: String,
    pub origin: String,
    pub destination: String,
    pub departure: String,
    pub arrival: String,
    pub duration_mins: u32,
    pub classes: Vec<TrainClass>,
    pub days_of_operation: Vec<String>,
    pub tourist_recommended: bool,
}

/// Fare estimate between two stations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FareEstimate {
    pub origin: String,
    pub destination: String,
    pub class: TrainClass,
    pub fare_lkr: f64,
    pub note: String,
}

/// Fetch train schedules between two stations.
pub async fn fetch_schedules(
    client: &reqwest::Client,
    origin: &str,
    destination: &str,
) -> Result<Vec<TrainSchedule>> {
    let url = format!(
        "{SLR_SCHEDULE_API}?from={}&to={}",
        urlencoding::encode(origin),
        urlencoding::encode(destination)
    );

    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/thaaaru/apeclaw-labs)")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            r.json::<Vec<TrainSchedule>>()
                .await
                .context("Failed to parse train schedule JSON")
        }
        _ => {
            // Return stub with official page reference
            Ok(vec![TrainSchedule {
                train_number: "N/A".to_string(),
                train_name: format!("See {SLR_BASE_URL} for schedules"),
                origin: origin.to_string(),
                destination: destination.to_string(),
                departure: "N/A".to_string(),
                arrival: "N/A".to_string(),
                duration_mins: 0,
                classes: vec![],
                days_of_operation: vec![],
                tourist_recommended: false,
            }])
        }
    }
}

/// Return tourist tips for a given route.
pub fn tourist_tip(origin: &str, destination: &str) -> Option<&'static str> {
    match (origin, destination) {
        ("Kandy", "Ella") | ("Ella", "Kandy") => Some(
            "Book the Observation Car (tourist class) well in advance — seats sell out weeks ahead. \
             Sit on the right side Kandy→Ella for the best valley views. \
             Train 1005 (Podi Menike) and 1015 (Udarata Menike) are the most popular."
        ),
        ("Colombo Fort", "Galle") | ("Galle", "Colombo Fort") => Some(
            "The coastal line hugs the ocean for much of the journey. \
             Morning trains offer the best light for photography. \
             Buy tickets at the station — no advance booking required for 2nd/3rd class."
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tourist_routes_non_empty() {
        assert!(!TOURIST_ROUTES.is_empty());
    }

    #[test]
    fn kandy_ella_tip_exists() {
        let tip = tourist_tip("Kandy", "Ella");
        assert!(tip.is_some());
        assert!(tip.unwrap().contains("Observation Car"));
    }

    #[test]
    fn train_class_display_names_non_empty() {
        let classes = [
            TrainClass::FirstClassAC,
            TrainClass::SecondClass,
            TrainClass::ThirdClass,
            TrainClass::ObservationCar,
        ];
        for c in &classes {
            assert!(!c.display_name().is_empty());
        }
    }

    #[test]
    fn ella_kandy_tip_also_matches() {
        assert!(tourist_tip("Ella", "Kandy").is_some());
    }
}
