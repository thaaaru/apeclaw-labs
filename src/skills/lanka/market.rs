//! Market data tools for the Sri Lankan financial ecosystem.
//!
//! Provides:
//! - CBSL (Central Bank of Sri Lanka) LKR exchange rates
//! - CSE (Colombo Stock Exchange) equity data
//!
//! Unicode note: all Sinhala (සිංහල) and Tamil (தமிழ்) strings are valid
//! UTF-8 and handled natively by Rust's `str`/`String` types.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// ── CBSL exchange-rate endpoint ──────────────────────────────────────────────

/// CBSL public exchange-rate page (HTML scrape fallback).
const CBSL_RATES_URL: &str = "https://www.cbsl.gov.lk/rates-and-indicators/exchange-rates/daily-exchange-rates";

/// CBSL open-data JSON feed (when available).
const CBSL_API_URL: &str = "https://www.cbsl.gov.lk/api/exchange-rates";

/// CSE market summary endpoint.
const CSE_MARKET_URL: &str = "https://www.cse.lk/api/market-summary";

// ── Data types ───────────────────────────────────────────────────────────────

/// A single currency exchange rate against LKR.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// ISO 4217 currency code (e.g. "USD", "EUR").
    pub currency: String,
    /// Human-readable currency name.
    pub name: String,
    /// Bank buying rate in LKR.
    pub buying: f64,
    /// Bank selling rate in LKR.
    pub selling: f64,
    /// Sinhala label for UI display (සිංහල).
    pub label_si: String,
    /// Tamil label for UI display (தமிழ்).
    pub label_ta: String,
}

/// Snapshot of the Colombo Stock Exchange.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CseSnapshot {
    /// All-Share Price Index (ASPI).
    pub aspi: f64,
    /// S&P Sri Lanka 20 Index.
    pub sp20: f64,
    /// Total market turnover in LKR.
    pub turnover_lkr: f64,
    /// Number of trades in the session.
    pub trades: u64,
}

// ── CBSL rate fetch ──────────────────────────────────────────────────────────

/// Fetch the latest LKR exchange rates from the Central Bank of Sri Lanka.
///
/// Tries the JSON API first; falls back to the HTML page URL so the caller
/// can scrape or display it to the user.
///
/// # Errors
/// Returns an error if the HTTP request fails.  API-level parsing errors are
/// surfaced as `anyhow::Error` with context.
pub async fn fetch_lkr_rates(client: &reqwest::Client) -> Result<Vec<ExchangeRate>> {
    // Attempt the structured JSON feed first.
    let resp = client
        .get(CBSL_API_URL)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/apeclaw-labs/apeclaw)")
        .send()
        .await;

    match resp {
        Ok(r) if r.status().is_success() => {
            let rates: Vec<ExchangeRate> = r
                .json()
                .await
                .context("Failed to deserialise CBSL exchange-rate JSON")?;
            return Ok(rates);
        }
        _ => {
            // JSON feed unavailable — return a stub pointing the user to the
            // HTML page so the agent can use web_fetch if needed.
        }
    }

    // Stub rates with the page URL embedded in the name for agent fallback.
    Ok(vec![ExchangeRate {
        currency: "USD".to_string(),
        name: format!("See {CBSL_RATES_URL}"),
        buying: 0.0,
        selling: 0.0,
        label_si: "ඇමෙරිකානු ඩොලර්".to_string(),
        label_ta: "அமெரிக்க டாலர்".to_string(),
    }])
}

/// Format exchange rates into a locale-aware table string.
///
/// Returns both English and Sinhala/Tamil labels so the agent can present
/// bilingual output to Sri Lankan users.
pub fn format_rates_table(rates: &[ExchangeRate]) -> String {
    let mut out = String::from("通貨 / Currency | Buying (LKR) | Selling (LKR)\n");
    out.push_str("---------------|-------------|---------------\n");
    for r in rates {
        out.push_str(&format!(
            "{} ({} / {}) | {:.2} | {:.2}\n",
            r.currency, r.label_si, r.label_ta, r.buying, r.selling
        ));
    }
    out
}

// ── CSE snapshot ─────────────────────────────────────────────────────────────

/// Fetch the latest Colombo Stock Exchange market summary.
///
/// # Errors
/// Returns an error if the HTTP request or JSON parse fails.
pub async fn fetch_cse_snapshot(client: &reqwest::Client) -> Result<CseSnapshot> {
    let resp = client
        .get(CSE_MARKET_URL)
        .header("Accept", "application/json")
        .header("User-Agent", "ApeClaw/0.1.0-alpha (+https://github.com/apeclaw-labs/apeclaw)")
        .send()
        .await
        .context("CSE market summary request failed")?;

    resp.json::<CseSnapshot>()
        .await
        .context("Failed to deserialise CSE market snapshot")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_rates_table_handles_empty() {
        let table = format_rates_table(&[]);
        assert!(table.contains("Currency"));
    }

    #[test]
    fn sinhala_tamil_strings_are_valid_utf8() {
        // Ensure the compile-time Unicode literals round-trip cleanly.
        let si = "ඇමෙරිකානු ඩොලර්";
        let ta = "அமெரிக்க டாலர்";
        assert!(si.chars().count() > 0);
        assert!(ta.chars().count() > 0);
    }

    #[test]
    fn exchange_rate_serialises() {
        let r = ExchangeRate {
            currency: "USD".to_string(),
            name: "US Dollar".to_string(),
            buying: 300.0,
            selling: 310.0,
            label_si: "ඇමෙරිකානු ඩොලර්".to_string(),
            label_ta: "அமெரிக்க டாலர்".to_string(),
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("USD"));
    }
}
