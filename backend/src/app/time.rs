pub(super) use chrono::NaiveDate;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct DateRange {
    start: NaiveDate,
    end: NaiveDate,
}

impl DateRange {
    pub(crate) fn new(start: NaiveDate, end: NaiveDate) -> Self {
        assert!(
            start <= end,
            "Expect the start date to be earlier than the end date \
             when constructing `DateRange`, got start={start}, end={end}"
        );
        Self { start, end }
    }

    pub(crate) fn start(&self) -> NaiveDate {
        self.start
    }

    pub(crate) fn end(&self) -> NaiveDate {
        self.end
    }

    #[allow(dead_code)]
    pub(crate) fn into_start_end(self) -> (NaiveDate, NaiveDate) {
        (self.start(), self.end())
    }

    pub(crate) fn contains(&self, range: DateRange) -> bool {
        self.start() <= range.start() && range.end() <= self.end()
    }

    #[allow(dead_code)]
    pub(crate) fn intersects(&self, range: DateRange) -> bool {
        self.start() < range.end() && range.start() < self.end()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub(crate) enum Epoch {
    Date(#[serde(with = "serde_naive_date")] NaiveDate),
    Week(Week),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub(crate) struct Week {
    #[serde(with = "serde_naive_date")]
    start_date: NaiveDate,
}

impl Week {
    pub(crate) fn from_start_date(date: NaiveDate) -> Self {
        Self { start_date: date }
    }

    fn end_date(&self) -> NaiveDate {
        self.start_date + TimeDelta::weeks(1)
    }
}

pub(crate) trait EpochLike {
    fn start_date(&self) -> NaiveDate;
    fn end_date(&self) -> NaiveDate;
    fn date_range(&self) -> DateRange {
        DateRange {
            start: self.start_date(),
            end: self.end_date(),
        }
    }
    fn index_date(&self) -> NaiveDate;
}

impl EpochLike for NaiveDate {
    fn start_date(&self) -> NaiveDate {
        *self
    }
    fn end_date(&self) -> NaiveDate {
        *self + TimeDelta::days(1)
    }
    fn index_date(&self) -> NaiveDate {
        *self
    }
}

impl EpochLike for Week {
    fn start_date(&self) -> NaiveDate {
        self.start_date
    }
    fn end_date(&self) -> NaiveDate {
        self.start_date() + TimeDelta::weeks(1)
    }
    fn index_date(&self) -> NaiveDate {
        self.start_date
    }
}

impl EpochLike for Epoch {
    fn start_date(&self) -> NaiveDate {
        match self {
            Epoch::Date(d) => d.start_date(),
            Epoch::Week(w) => w.start_date(),
        }
    }
    fn end_date(&self) -> NaiveDate {
        match self {
            Epoch::Date(d) => d.end_date(),
            Epoch::Week(w) => w.end_date(),
        }
    }
    fn index_date(&self) -> NaiveDate {
        match self {
            Epoch::Date(d) => d.index_date(),
            Epoch::Week(w) => w.index_date(),
        }
    }
}

impl Epoch {
    pub(crate) fn contains(&self, other: Epoch) -> bool {
        self.date_range().contains(other.date_range())
    }
}

mod serde_naive_date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    use super::*;

    #[googletest::test]
    fn week_epoch_contains_last_day() {
        let week = Epoch::Week(Week {
            // 2024-09-15 is Monday
            start_date: NaiveDate::from_ymd_opt(2024, 9, 9).unwrap(),
        });

        expect_true!(week.contains(Epoch::Date(NaiveDate::from_ymd_opt(2024, 9, 15).unwrap())));
    }
}
