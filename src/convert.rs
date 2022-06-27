use chinese_lunisolar_calendar::chrono::prelude::*;

use chinese_lunisolar_calendar::{LunisolarDate, SolarDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    year: Option<u32>,
    month: u32,
    day: u32,
}

pub fn solar_to_lunar(year: u16, month: u8, day: u8) -> Option<DateResponse> {
    match SolarDate::from_ymd(year, month, day) {
        Ok(solar_date) => match solar_date.to_lunisolar_date() {
            Ok(lunar_date) => Some(DateResponse {
                year: None,
                month: lunar_date.get_lunar_month().to_u8() as u32,
                day: lunar_date.get_lunar_day().to_u8() as u32,
            }),
            Err(_) => None,
        },
        Err(_) => None,
    }
}

pub fn lunar_to_solar(year: u16, month: u8, day: u8) -> Vec<DateResponse> {
    [false, true]
        .into_iter()
        .filter_map(|leap| LunisolarDate::from_ymd(year, month, leap, day).ok())
        .map(|o| {
            let d = o.to_solar_date().to_naive_date();
            DateResponse {
                year: Some(d.year() as u32),
                month: d.month() as u32,
                day: d.day() as u32,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_to_lunar() {
        let lunar = solar_to_lunar(1991, 6, 6).unwrap();
        assert_eq!(lunar.month, 4);
        assert_eq!(lunar.day, 24)
    }

    #[test]
    fn test_solar_to_lunar_cant_convert_invalid_solar_date() {
        assert!(solar_to_lunar(2000, 0, 0).is_none())
    }

    #[test]
    fn test_solar_to_lunar_cant_convert_invalid_lunar_date() {
        // Out of range for LunisolarDate
        assert!(solar_to_lunar(1000, 1, 1).is_none())
    }

    #[test]
    fn test_lunar_to_solar_leap() {
        let out = lunar_to_solar(2020, 1, 1);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].month, 1);
        assert_eq!(out[0].day, 25);
    }

    #[test]
    fn test_lunar_to_solar_leap_month() {
        let out = lunar_to_solar(2020, 4, 4);

        assert_eq!(out.len(), 2);
        assert_eq!(out[0].month, 4);
        assert_eq!(out[0].day, 26);
        assert_eq!(out[1].month, 5);
        assert_eq!(out[1].day, 26)
    }
}
