//! Sri Lanka cultural calendar — public holidays, Poya days, and festivals.
//!
//! Poya days (full moon) are public holidays in Sri Lanka. Most businesses,
//! tourist sites, and alcohol sales are restricted. Tourists frequently get
//! caught out — this module helps agents warn and inform proactively.

use serde::{Deserialize, Serialize};

/// A public holiday or cultural event in Sri Lanka.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalEvent {
    pub name: String,
    pub name_si: String,
    pub name_ta: String,
    pub date: String,
    pub event_type: EventType,
    pub tourist_impact: TouristImpact,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    /// Full-moon Poya day (public holiday)
    Poya,
    /// National public holiday
    NationalHoliday,
    /// Religious festival (may not be a holiday)
    Festival,
    /// Cultural/seasonal event
    Cultural,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TouristImpact {
    /// Alcohol sales banned, many businesses closed
    High,
    /// Some closures, reduced services
    Medium,
    /// Crowds and traffic expected
    Low,
    /// Positive — great time to visit
    Positive,
}

impl TouristImpact {
    pub fn advisory(&self) -> &str {
        match self {
            TouristImpact::High => "Alcohol sales banned island-wide. Many shops and restaurants closed. Plan ahead.",
            TouristImpact::Medium => "Some businesses closed. Reduced transport services possible.",
            TouristImpact::Low => "Expect crowds and traffic in major cities and sites.",
            TouristImpact::Positive => "Excellent time to experience Sri Lankan culture. Book accommodation early.",
        }
    }
}

/// Return the fixed public holidays for Sri Lanka (year-independent dates).
/// Poya dates vary annually — use `poya_dates_2026()` for specific years.
pub fn fixed_public_holidays() -> Vec<CulturalEvent> {
    vec![
        CulturalEvent {
            name: "Tamil Thai Pongal Day".to_string(),
            name_si: "தமிழ் தை பொங்கல் நாள்".to_string(),
            name_ta: "தமிழ் தை பொங்கல் நாள்".to_string(),
            date: "January 14".to_string(),
            event_type: EventType::NationalHoliday,
            tourist_impact: TouristImpact::Positive,
            description: "Tamil harvest festival. Celebrated with kolam, sugarcane, and sweet rice.".to_string(),
        },
        CulturalEvent {
            name: "Independence Day".to_string(),
            name_si: "නිදහස් දිනය".to_string(),
            name_ta: "சுதந்திர தினம்".to_string(),
            date: "February 4".to_string(),
            event_type: EventType::NationalHoliday,
            tourist_impact: TouristImpact::Positive,
            description: "Sri Lanka's independence from British rule (1948). Parade in Colombo.".to_string(),
        },
        CulturalEvent {
            name: "Sinhala & Tamil New Year".to_string(),
            name_si: "සිංහල හා දෙමළ අලුත් අවුරුදු".to_string(),
            name_ta: "சிங்கள தமிழ் புத்தாண்டு".to_string(),
            date: "April 13–14".to_string(),
            event_type: EventType::NationalHoliday,
            tourist_impact: TouristImpact::High,
            description: "The most important cultural celebration. Country-wide festivities. \
                          Transport and accommodation fully booked. Many businesses closed for a week.".to_string(),
        },
        CulturalEvent {
            name: "Vesak Full Moon Poya".to_string(),
            name_si: "වෙසක් පූර්ණිමා".to_string(),
            name_ta: "வேசாக் பௌர்ணமி".to_string(),
            date: "May (full moon — varies)".to_string(),
            event_type: EventType::Poya,
            tourist_impact: TouristImpact::High,
            description: "Most sacred Buddhist holiday. Lanterns, dansalas (free food stalls), \
                          pandols island-wide. Alcohol banned. Stunning atmosphere for tourists.".to_string(),
        },
        CulturalEvent {
            name: "Esala Perahera".to_string(),
            name_si: "එසල පෙරහැර".to_string(),
            name_ta: "ஆடி பெரகரா".to_string(),
            date: "July–August (10 days, Kandy)".to_string(),
            event_type: EventType::Festival,
            tourist_impact: TouristImpact::Positive,
            description: "One of Asia's grandest festivals. Elephants, dancers, drummers parade \
                          through Kandy for the Sacred Tooth Relic. Book Kandy accommodation \
                          6+ months in advance.".to_string(),
        },
        CulturalEvent {
            name: "Deepavali".to_string(),
            name_si: "දීපාවලිය".to_string(),
            name_ta: "தீபாவளி".to_string(),
            date: "October/November (varies)".to_string(),
            event_type: EventType::NationalHoliday,
            tourist_impact: TouristImpact::Positive,
            description: "Festival of Lights. Celebrated by Tamil community. Oil lamps, fireworks, sweets.".to_string(),
        },
        CulturalEvent {
            name: "Christmas Day".to_string(),
            name_si: "නත්තල් දිනය".to_string(),
            name_ta: "கிறிஸ்மஸ் தினம்".to_string(),
            date: "December 25".to_string(),
            event_type: EventType::NationalHoliday,
            tourist_impact: TouristImpact::Low,
            description: "Public holiday. Peak tourist season — beaches packed, prices high.".to_string(),
        },
    ]
}

/// Return all Poya (full moon) dates for 2026.
pub fn poya_dates_2026() -> Vec<(&'static str, &'static str)> {
    vec![
        ("January 13",  "Duruthu Poya"),
        ("February 12", "Navam Poya"),
        ("March 13",    "Medin Poya"),
        ("April 12",    "Bak Poya"),
        ("May 12",      "Vesak Poya — most sacred"),
        ("June 11",     "Poson Poya"),
        ("July 10",     "Esala Poya"),
        ("August 9",    "Nikini Poya"),
        ("September 7", "Binara Poya"),
        ("October 6",   "Vap Poya"),
        ("November 5",  "Il Poya"),
        ("December 4",  "Unduvap Poya"),
    ]
}

/// Check if a given date string matches a Poya or major holiday.
/// Returns a tourist advisory if it does.
pub fn check_date_advisory(date_str: &str) -> Option<String> {
    let date_lower = date_str.to_lowercase();
    for (poya_date, poya_name) in poya_dates_2026() {
        if date_lower.contains(&poya_date.to_lowercase()) {
            return Some(format!(
                "⚠️  {} is a Poya day ({}). Alcohol sales banned island-wide. \
                 Many shops closed. Plan purchases and restaurant bookings in advance.",
                poya_date, poya_name
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_holidays_non_empty() {
        let holidays = fixed_public_holidays();
        assert!(!holidays.is_empty());
    }

    #[test]
    fn poya_2026_has_12_entries() {
        assert_eq!(poya_dates_2026().len(), 12);
    }

    #[test]
    fn vesak_is_most_sacred() {
        let poya = poya_dates_2026();
        let vesak = poya.iter().find(|(_, name)| name.contains("Vesak"));
        assert!(vesak.is_some());
    }

    #[test]
    fn check_date_advisory_finds_poya() {
        let advisory = check_date_advisory("May 12");
        assert!(advisory.is_some());
        assert!(advisory.unwrap().contains("Poya"));
    }

    #[test]
    fn check_date_advisory_none_for_normal_day() {
        assert!(check_date_advisory("June 15").is_none());
    }

    #[test]
    fn tourist_impact_advisory_non_empty() {
        for impact in [
            TouristImpact::High,
            TouristImpact::Medium,
            TouristImpact::Low,
            TouristImpact::Positive,
        ] {
            assert!(!impact.advisory().is_empty());
        }
    }

    #[test]
    fn sinhala_tamil_labels_valid_utf8() {
        let events = fixed_public_holidays();
        for e in &events {
            assert!(e.name_si.chars().count() > 0);
            assert!(e.name_ta.chars().count() > 0);
        }
    }
}
