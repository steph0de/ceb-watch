use crate::{native_date_time_range::NaiveDateTimeRange, utils};
use anyhow::{Context, Result};
use log::warn;
use regex::Regex;
use std::collections::HashMap;

extern crate table_extract;

pub struct PowerOutages(HashMap<String, Region>);

impl PowerOutages {
    pub fn new(data: String) -> Self {
        let data: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        let mut po: HashMap<String, Region> = HashMap::new();
        for d in data {
            let v = Region::new(d.1);
            po.insert(d.0, v);
        }
        PowerOutages(po)
    }
    pub fn get_region(&self, region_name: String) -> Result<Region, anyhow::Error> {
        let s = &self.0;
        let h = s.get(&region_name).context("")?;
        Ok(h.clone())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
pub struct Region(Vec<PowerOutage>);

impl Region {
    #[warn(dead_code)]
    fn new(region_data_html: String) -> Self {
        let table = table_extract::Table::find_first(&region_data_html).unwrap();
        let mut region_data = Vec::new();
        for row in table.into_iter() {
            if row.is_empty() || row.get("Date").unwrap().is_empty() {
                warn!("Row is empty")
            } else {
                region_data.push(PowerOutage {
                    date: utils::parse_from_str(row.get("Date").unwrap()).unwrap(),
                    locality: String::from(row.get("Locality").unwrap_or("<locality missing>")),
                    streets: String::from(row.get("Streets").unwrap_or("<streets missing>")),
                });
            }
        }
        Region(region_data)
    }
    pub fn get_poweroutages(&self) -> Vec<PowerOutage> {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct PowerOutage {
    date: NaiveDateTimeRange,
    locality: String,
    streets: String,
}

impl PowerOutage {
    fn new(date: NaiveDateTimeRange, locality: String, streets: String) -> Self {
        Self {
            date,
            locality,
            streets,
        }
    }

    pub fn date(&self) -> &NaiveDateTimeRange {
        &self.date
    }
    pub fn locality(&self) -> &str {
        self.locality.as_ref()
    }

    pub fn streets(&self) -> &str {
        self.streets.as_ref()
    }
}

/// Get Power Outage Information in "JSON" string format
pub fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let power_outage_url: &str = "https://ceb.mu/customer-corner/power-outage-information";
    let body = attohttpc::get(power_outage_url).send()?.text()?;
    let re = Regex::new(r"var arDistrictLocations = (.*);").unwrap();
    match re.captures(&body.to_owned()) {
        Some(caps) => Ok(caps[1].to_string()),
        None => return Err("Could not find any matches.".into()),
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use chrono::NaiveDateTime;

    use super::*;

    #[test]
    fn test_cebdata(){
        let data = r#"{"blackriver":"<h3>Black River<\/h3>\r\n<table class=\"table table-sm fs-12\" id=\"table-mauritius-blackriver\" class=\"display:none;\">\r\n    <thead>\r\n        <th class=\"col bt-0\" style=\"width: 45%\">Date<\/th>\r\n        <th class=\"col bt-0\" style=\"width: 15%\">Locality<\/th>\r\n        <th class=\"col bt-0\" style=\"width: 40%\">Streets<\/th>\r\n    <\/thead>\r\n    <tbody>\r\n        \r\n<tr>\r\n    <td class=\"align-top\">Le mardi 16 mai 2023 de  08:30:00 \u00e0  16:30:00<\/td>\r\n    <td class=\"align-top\">RIVIERE NOIRE<\/td>\r\n    <td class=\"align-top\">MORC RAMDANEE<\/td>\r\n<\/tr>\r\n\r\n<tr>\r\n    <td class=\"align-top\"><\/td>\r\n    <td class=\"align-top\"><\/td>\r\n    <td class=\"align-top\"><\/td>\r\n<\/tr>\r\n\r\n    <\/tbody>\r\n<\/table>\r\n"}"#;
        
        let pos = PowerOutages::new(data.to_string());
        assert_eq!(pos.len(),1);
        let region = pos.get_region("blackriver".to_string());
        let po = &region.unwrap().get_poweroutages().get(0).unwrap().clone();
        assert_eq!(po.date.get_start(), NaiveDateTime::from_str("2023-05-06T08:30:00").unwrap());
        assert_eq!(po.locality, "RIVIERE NOIRE");
        // let p = region.unwrap().get_poweroutages();
        // println!("{:?}", p);
    }
}
