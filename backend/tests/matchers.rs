use std::{collections::HashMap};

use googletest::{
    description::Description,
    matcher::MatcherResult,
    matchers::pat,
    prelude::{Matcher, MatcherBase},
};
use uuid::Uuid;

pub fn json_null() -> impl for<'a> Matcher<&'a serde_json::Value> {
    pat!(serde_json::Value::Null)
}

#[macro_export]
macro_rules! json_obj {
    ( $( $key:ident: $val:expr),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $({
                let val_matcher: Box<dyn for<'a> Matcher<&'a serde_json::Value>> = Box::new($val);
                map.insert(stringify!($key).to_owned(), val_matcher);
            })*

            $crate::matchers::JsonObjMatcher { expect: map }
        }
    };
}

#[derive(MatcherBase)]
pub struct JsonObjMatcher {
    pub expect: HashMap<String, Box<dyn for<'a> Matcher<&'a serde_json::Value>>>,
}

impl<'a> Matcher<&'a serde_json::Value> for JsonObjMatcher {
    fn matches(&self, actual: &'a serde_json::Value) -> MatcherResult {
        let serde_json::Value::Object(map) = actual else {
            return MatcherResult::NoMatch;
        };
        if self.expect.len() != map.len() {
            return MatcherResult::NoMatch;
        }

        for (k, v) in self.expect.iter() {
            if !map.contains_key(k) {
                return MatcherResult::NoMatch;
            }
            if v.matches(&map[k]).is_no_match() {
                return MatcherResult::NoMatch;
            }
        }

        MatcherResult::Match
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        let prefix = match matcher_result {
            MatcherResult::Match => "is",
            MatcherResult::NoMatch => "is not",
        }
        .to_owned();

        if self.expect.is_empty() {
            return format!("{} an empty JSON object", prefix).into();
        }

        format!(
            "{} a JSON object with these key-value pair(s):\n{}",
            prefix,
            self.expect
                .iter()
                .map(|(key, val)| {
                    format!(
                        "key \"{}\" with value that {}",
                        key,
                        val.describe(matcher_result),
                    )
                })
                .collect::<Description>()
                .bullet_list()
                .indent()
        ).into()
    }
}

pub fn json_string(
    matcher: impl for<'a> Matcher<&'a String>,
) -> impl for<'a> Matcher<&'a serde_json::Value> {
    pat!(serde_json::Value::String(matcher))
}

#[derive(MatcherBase)]
pub struct JsonU64Matcher<T>
where
    T: Matcher<u64>,
{
    pub expect: T,
}

impl<'a, T> Matcher<&'a serde_json::Value> for JsonU64Matcher<T>
where
    T: Matcher<u64>,
{
    fn matches(&self, actual: &'a serde_json::Value) -> MatcherResult {
        let serde_json::Value::Number(number) = actual else {
            return MatcherResult::NoMatch;
        };

        let Some(number) = number.as_u64() else {
            return MatcherResult::NoMatch;
        };

        self.expect.matches(number)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        let prefix = match matcher_result {
            MatcherResult::Match => "is",
            MatcherResult::NoMatch => "is not",
        };

        format!(
            "{} a JSON number, which {}",
            prefix,
            self.expect.describe(matcher_result)
        )
        .into()
    }
}

pub fn json_u64(matcher: impl Matcher<u64>) -> impl for<'a> Matcher<&'a serde_json::Value> {
    JsonU64Matcher { expect: matcher }
}

#[derive(MatcherBase)]
pub struct UuidStrMatcher<ExpectedM>
where
    ExpectedM: Matcher<Uuid>,
{
    expect: ExpectedM,
}

impl<ExpectedM, ActualT> Matcher<ActualT> for UuidStrMatcher<ExpectedM>
where
    ExpectedM: Matcher<Uuid>,
    ActualT: AsRef<str> + std::fmt::Debug + Copy,
{
    fn matches(&self, actual: ActualT) -> MatcherResult {
        let Ok(uuid) = Uuid::parse_str(actual.as_ref()) else {
            return MatcherResult::NoMatch;
        };

        self.expect.matches(uuid)
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        let prefix = match matcher_result {
            MatcherResult::Match => "is",
            MatcherResult::NoMatch => "is not",
        };
        format!(
            "{} a UUID string that {}",
            prefix,
            self.expect.describe(matcher_result)
        )
        .into()
    }
}

pub fn uuid_str<ExpectedM>(matcher: ExpectedM) -> UuidStrMatcher<ExpectedM>
where
    ExpectedM: Matcher<Uuid>,
{
    UuidStrMatcher { expect: matcher }
}
