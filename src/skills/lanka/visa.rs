//! Sri Lanka Electronic Travel Authorization (ETA) visa checker.
//!
//! Source: <https://www.eta.gov.lk>
//!
//! Most tourists require an ETA before arrival. This module provides
//! eligibility checking and application status lookup.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const ETA_BASE_URL: &str = "https://www.eta.gov.lk";
const ETA_STATUS_API: &str = "https://www.eta.gov.lk/slvisa/visainfo/api/status";

/// Visa-free countries for Sri Lanka (short stay, no ETA required).
/// Source: Department of Immigration and Emigration, 2025.
pub const VISA_FREE_COUNTRIES: &[&str] = &[
    "Maldives", "Seychelles", "Singapore",
];

/// Countries eligible for free ETA (tourist).
pub const FREE_ETA_COUNTRIES: &[&str] = &[
    "Australia", "Austria", "Belgium", "Canada", "Denmark", "Finland",
    "France", "Germany", "Greece", "Ireland", "Italy", "Japan", "Luxembourg",
    "Netherlands", "New Zealand", "Norway", "Portugal", "Spain", "Sweden",
    "Switzerland", "United Kingdom", "United States",
    // SAARC countries
    "Bangladesh", "Bhutan", "India", "Nepal", "Pakistan",
];

/// ETA visa types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EtaType {
    /// Short visit (up to 30 days, extendable to 6 months)
    ShortVisit,
    /// Transit (up to 2 days)
    Transit,
    /// Double entry
    DoubleEntry,
}

impl EtaType {
    pub fn display(&self) -> &str {
        match self {
            EtaType::ShortVisit  => "Short Visit (30 days, extendable)",
            EtaType::Transit     => "Transit (48 hours)",
            EtaType::DoubleEntry => "Double Entry",
        }
    }
}

/// ETA application status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtaStatus {
    pub reference_number: String,
    pub status: String,
    pub visa_type: Option<EtaType>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub passport_number: String,
    pub note: Option<String>,
}

/// Eligibility result for a nationality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisaEligibility {
    pub nationality: String,
    pub requires_eta: bool,
    pub eta_is_free: bool,
    pub eta_fee_usd: Option<f64>,
    pub max_stay_days: u32,
    pub advisory: String,
    pub apply_url: String,
}

/// Check visa eligibility for a given nationality.
pub fn check_eligibility(nationality: &str) -> VisaEligibility {
    let norm = nationality.trim();

    if VISA_FREE_COUNTRIES.iter().any(|c| c.eq_ignore_ascii_case(norm)) {
        return VisaEligibility {
            nationality: norm.to_string(),
            requires_eta: false,
            eta_is_free: false,
            eta_fee_usd: None,
            max_stay_days: 30,
            advisory: "No ETA required. Free entry on arrival for up to 30 days.".to_string(),
            apply_url: ETA_BASE_URL.to_string(),
        };
    }

    if FREE_ETA_COUNTRIES.iter().any(|c| c.eq_ignore_ascii_case(norm)) {
        return VisaEligibility {
            nationality: norm.to_string(),
            requires_eta: true,
            eta_is_free: true,
            eta_fee_usd: Some(0.0),
            max_stay_days: 30,
            advisory: "Free ETA required. Apply online before travel. \
                       Extendable up to 6 months at the Department of Immigration in Colombo."
                .to_string(),
            apply_url: format!("{ETA_BASE_URL}/slvisa/"),
        };
    }

    // Default: paid ETA
    VisaEligibility {
        nationality: norm.to_string(),
        requires_eta: true,
        eta_is_free: false,
        eta_fee_usd: Some(20.0),
        max_stay_days: 30,
        advisory: format!(
            "ETA required (USD 20). Apply at {ETA_BASE_URL} before travel. \
             Processing time: 24–72 hours. Extendable on arrival."
        ),
        apply_url: format!("{ETA_BASE_URL}/slvisa/"),
    }
}

/// Fetch ETA application status by reference number and passport.
pub async fn fetch_eta_status(
    client: &reqwest::Client,
    reference_number: &str,
    passport_number: &str,
) -> Result<EtaStatus> {
    let url = format!(
        "{ETA_STATUS_API}?ref={}&passport={}",
        urlencoding::encode(reference_number),
        urlencoding::encode(passport_number)
    );

    let resp = client
        .get(&url)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/thaaaru/apeclaw-labs)")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            r.json::<EtaStatus>()
                .await
                .context("Failed to parse ETA status response")
        }
        _ => Ok(EtaStatus {
            reference_number: reference_number.to_string(),
            passport_number: passport_number.to_string(),
            status: format!("Check status at {ETA_BASE_URL}/slvisa/visainfo/checkstatus"),
            visa_type: None,
            valid_from: None,
            valid_until: None,
            note: Some("API unavailable. Please check status directly on the ETA portal.".to_string()),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uk_nationals_get_free_eta() {
        let result = check_eligibility("United Kingdom");
        assert!(result.requires_eta);
        assert!(result.eta_is_free);
        assert_eq!(result.eta_fee_usd, Some(0.0));
    }

    #[test]
    fn maldives_visa_free() {
        let result = check_eligibility("Maldives");
        assert!(!result.requires_eta);
    }

    #[test]
    fn unknown_country_requires_paid_eta() {
        let result = check_eligibility("Ruritania");
        assert!(result.requires_eta);
        assert!(!result.eta_is_free);
        assert_eq!(result.eta_fee_usd, Some(20.0));
    }

    #[test]
    fn case_insensitive_lookup() {
        let result = check_eligibility("united states");
        assert!(result.eta_is_free);
    }

    #[test]
    fn apply_url_contains_eta_domain() {
        let result = check_eligibility("Germany");
        assert!(result.apply_url.contains("eta.gov.lk"));
    }

    #[test]
    fn eta_type_display_non_empty() {
        for t in [EtaType::ShortVisit, EtaType::Transit, EtaType::DoubleEntry] {
            assert!(!t.display().is_empty());
        }
    }
}
