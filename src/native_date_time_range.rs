use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct NaiveDateTimeRange {
    /// Start of the DateTime range
    start: NaiveDateTime,
    /// End of the DateTime range
    end: NaiveDateTime,
}

impl NaiveDateTimeRange {
    /// Create a [`NaiveDateTimeRange`] using two [`NaiveDateTime`].
    pub fn new_from_range(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }
    /// Returns the starting datetime as [`NaiveDateTime`].
    pub fn get_start(&self) -> NaiveDateTime {
        self.start
    }
    /// Returns the ending datetime as [`NaiveDateTime`].
    pub fn get_end(&self) -> NaiveDateTime {
        self.end
    }
    /// Set the timerange's start date & time
    pub fn set_start(&mut self, start: NaiveDateTime) {
        self.start = start;
    }

    /// Set the timerange's end date & time
    pub fn set_end(&mut self, end: NaiveDateTime) {
        self.end = end;
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate};

    use super::*;

    #[test]
    fn test_range() {
        let s = NaiveDate::from_ymd_opt(2022, 7, 8)
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap();
        let e = NaiveDate::from_ymd_opt(2022, 7, 8)
            .unwrap()
            .and_hms_opt(17, 00, 05)
            .unwrap();
        let dtrg = NaiveDateTimeRange::new_from_range(s, e);
        assert_eq!(dtrg.start.to_string(), "2022-07-08 09:10:11");
        assert_eq!(dtrg.end.to_string(), "2022-07-08 17:00:05");
        assert_eq!(dtrg.get_start().year(), 2022);
    }
}
