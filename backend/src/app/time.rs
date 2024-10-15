pub(super) use chrono::NaiveDate;
use chrono::{Local, TimeDelta};
use serde::{Deserialize, Serialize};

pub(crate) fn today() -> NaiveDate {
    Local::now().date_naive()
}

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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum Epoch {
    Date(#[serde(with = "serde_naive_date")] NaiveDate),
    Week(Week),
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum EpochKind {
    Date,
    Week,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) struct RecurringSpec {
    pub(crate) start_date: NaiveDate,
    pub(crate) pattern: RecurringPattern,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) enum RecurringPattern {
    EveryEpoch { kind: EpochKind, every: i32 },
}

impl RecurringSpec {
    pub(crate) fn next_after(&self, date: NaiveDate) -> Epoch {
        match &self.pattern {
            RecurringPattern::EveryEpoch {
                kind: EpochKind::Date,
                every,
            } => {
                if date < self.start_date {
                    return Epoch::Date(self.start_date);
                }

                let date_diff = (date - self.start_date).num_days();
                let every = *every as i64;
                let next = self.start_date + TimeDelta::days((date_diff / every + 1) * every);
                Epoch::Date(next)
            }
            RecurringPattern::EveryEpoch {
                kind: EpochKind::Week,
                every,
            } => {
                if date < self.start_date {
                    return Epoch::Week(Week::from_start_date(self.start_date));
                }

                let week_diff = (date - self.start_date).num_weeks();
                let every = *every as i64;
                let next_week_start =
                    self.start_date + TimeDelta::weeks((week_diff / every + 1) * every);
                Epoch::Week(Week::from_start_date(next_week_start))
            }
        }
    }

    pub(crate) fn next_starting_from(&self, date: NaiveDate) -> Epoch {
        self.next_after(date - TimeDelta::days(1))
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

    #[googletest::test]
    fn recurring_week_next_after_a_day_is_next_n_week() {
        let recurring = RecurringSpec {
            start_date: NaiveDate::from_ymd_opt(2024, 9, 23).unwrap(), // Monday
            pattern: RecurringPattern::EveryEpoch {
                kind: EpochKind::Week,
                every: 2,
            },
        };

        expect_eq!(
            // The Monday exactly 2 weeks later.
            recurring.next_starting_from(NaiveDate::from_ymd_opt(2024, 10, 7).unwrap()),
            Epoch::Week(Week {
                start_date: NaiveDate::from_ymd_opt(2024, 10, 7).unwrap()
            })
        );
        expect_eq!(
            recurring.next_starting_from(NaiveDate::from_ymd_opt(2024, 10, 8).unwrap()),
            Epoch::Week(Week {
                start_date: NaiveDate::from_ymd_opt(2024, 10, 21).unwrap()
            })
        );
        expect_eq!(
            recurring.next_starting_from(NaiveDate::from_ymd_opt(2024, 10, 15).unwrap()),
            Epoch::Week(Week {
                start_date: NaiveDate::from_ymd_opt(2024, 10, 21).unwrap()
            })
        );
    }

    #[googletest::test]
    fn recurring_week_next_after_a_day_before_start_date_is_the_week_of_start_date() {
        let recurring = RecurringSpec {
            start_date: NaiveDate::from_ymd_opt(2024, 9, 23).unwrap(),
            pattern: RecurringPattern::EveryEpoch {
                kind: EpochKind::Week,
                every: 2,
            },
        };

        expect_eq!(
            recurring.next_starting_from(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            Epoch::Week(Week {
                // Should be the monday 4 weeks after
                start_date: NaiveDate::from_ymd_opt(2024, 9, 23).unwrap()
            })
        );
        expect_eq!(
            recurring.next_starting_from(NaiveDate::from_ymd_opt(2024, 9, 23).unwrap()),
            Epoch::Week(Week {
                // Should be the monday 4 weeks after
                start_date: NaiveDate::from_ymd_opt(2024, 9, 23).unwrap()
            })
        );
    }
}
