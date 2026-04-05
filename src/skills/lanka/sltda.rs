//! Sri Lanka Tourism Development Authority (SLTDA) licensed operator lookup.
//!
//! Source: <https://www.sltda.gov.lk>
//!
//! SLTDA licenses hotels, guesthouses, tour operators, tour guides, and
//! tourist vehicles. Checking SLTDA registration protects tourists from
//! unlicensed operators and scams.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const SLTDA_BASE_URL: &str = "https://www.sltda.gov.lk";
const SLTDA_SEARCH_API: &str = "https://www.sltda.gov.lk/api/search";

/// SLTDA-registered establishment types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EstablishmentType {
    Hotel,
    Guesthouse,
    Boutique,
    Homestay,
    TourOperator,
    TourGuide,
    TouristVehicle,
    Restaurant,
    SpaWellness,
}

impl EstablishmentType {
    pub fn display(&self) -> &str {
        match self {
            EstablishmentType::Hotel          => "Hotel",
            EstablishmentType::Guesthouse     => "Guesthouse",
            EstablishmentType::Boutique       => "Boutique Hotel",
            EstablishmentType::Homestay       => "Homestay",
            EstablishmentType::TourOperator   => "Licensed Tour Operator",
            EstablishmentType::TourGuide      => "Licensed Tour Guide",
            EstablishmentType::TouristVehicle => "Tourist Vehicle",
            EstablishmentType::Restaurant     => "Restaurant",
            EstablishmentType::SpaWellness    => "Spa & Wellness",
        }
    }
}

/// Star classification for hotels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarRating {
    One, Two, Three, Four, Five, Unclassified,
}

/// An SLTDA-registered tourism operator or establishment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SltdaOperator {
    pub registration_number: String,
    pub name: String,
    pub establishment_type: EstablishmentType,
    pub star_rating: Option<StarRating>,
    pub district: String,
    pub address: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub license_valid_until: Option<String>,
    pub is_active: bool,
}

/// Search parameters for SLTDA operator lookup.
#[derive(Debug, Clone)]
pub struct SltdaSearchParams {
    pub name: Option<String>,
    pub district: Option<String>,
    pub establishment_type: Option<EstablishmentType>,
    pub registration_number: Option<String>,
}

/// Search SLTDA registry for licensed operators.
///
/// Use this to verify whether a hotel, tour operator, or guide is
/// legitimately registered before booking.
pub async fn search_operators(
    client: &reqwest::Client,
    params: &SltdaSearchParams,
) -> Result<Vec<SltdaOperator>> {
    let mut query = String::new();
    if let Some(name) = &params.name {
        query.push_str(&format!("&name={}", urlencoding::encode(name)));
    }
    if let Some(district) = &params.district {
        query.push_str(&format!("&district={}", urlencoding::encode(district)));
    }
    if let Some(reg) = &params.registration_number {
        query.push_str(&format!("&reg={}", urlencoding::encode(reg)));
    }

    let url = format!("{SLTDA_SEARCH_API}?{}", query.trim_start_matches('&'));

    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/thaaaru/apeclaw-labs)")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            r.json::<Vec<SltdaOperator>>()
                .await
                .context("Failed to parse SLTDA search results")
        }
        _ => Ok(vec![SltdaOperator {
            registration_number: "N/A".to_string(),
            name: format!("Search at {SLTDA_BASE_URL}/en/registered-establishments"),
            establishment_type: EstablishmentType::Hotel,
            star_rating: None,
            district: "N/A".to_string(),
            address: String::new(),
            phone: None,
            email: None,
            website: Some(SLTDA_BASE_URL.to_string()),
            license_valid_until: None,
            is_active: false,
        }]),
    }
}

/// Generate a tourist safety advisory for booking accommodation.
pub fn booking_safety_advisory() -> &'static str {
    "Always verify your hotel or tour operator is SLTDA-registered before paying. \
     Licensed operators display their SLTDA registration certificate on-site. \
     Report unlicensed operators to SLTDA on +94 11 2 426900 or info@sltda.gov.lk. \
     Tourist Police hotline: 1912."
}

/// Key tourist areas in Sri Lanka with typical operator density.
pub fn tourist_areas() -> &'static [(&'static str, &'static str)] {
    &[
        ("Colombo",       "Commercial capital — business hotels, city tours"),
        ("Kandy",         "Cultural triangle — temples, hill country gateway"),
        ("Ella",          "Backpacker hub — hiking, tea country, scenic train"),
        ("Galle",         "Colonial fort, beaches, diving — southwest coast"),
        ("Mirissa",       "Whale watching, surfing, beaches"),
        ("Sigiriya",      "Rock fortress — UNESCO World Heritage Site"),
        ("Trincomalee",   "East coast beaches, diving — best Jun–Sep"),
        ("Arugam Bay",    "World-class surfing — best May–Oct"),
        ("Nuwara Eliya",  "Tea country, cool climate, colonial charm"),
        ("Anuradhapura",  "Ancient kingdom — UNESCO World Heritage Site"),
        ("Yala",          "Wildlife safari — leopards, elephants"),
        ("Bentota",       "Beach resort, water sports, Ayurveda"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tourist_areas_non_empty() {
        assert!(!tourist_areas().is_empty());
    }

    #[test]
    fn tourist_areas_contains_kandy() {
        assert!(tourist_areas().iter().any(|(area, _)| *area == "Kandy"));
    }

    #[test]
    fn booking_safety_advisory_non_empty() {
        assert!(!booking_safety_advisory().is_empty());
        assert!(booking_safety_advisory().contains("1912"));
    }

    #[test]
    fn establishment_type_display_non_empty() {
        let types = [
            EstablishmentType::Hotel,
            EstablishmentType::TourOperator,
            EstablishmentType::TourGuide,
            EstablishmentType::TouristVehicle,
        ];
        for t in &types {
            assert!(!t.display().is_empty());
        }
    }

    #[test]
    fn search_params_builder() {
        let params = SltdaSearchParams {
            name: Some("Cinnamon".to_string()),
            district: Some("Colombo".to_string()),
            establishment_type: Some(EstablishmentType::Hotel),
            registration_number: None,
        };
        assert_eq!(params.name.as_deref(), Some("Cinnamon"));
    }
}
