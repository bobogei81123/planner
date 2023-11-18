use std::{collections::HashMap, marker::PhantomData};

use googletest::{matcher::MatcherResult, pat, prelude::Matcher};
use uuid::Uuid;

#[macro_export]
macro_rules! json_obj {
    ( $( $key:ident: $val:expr),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $({
                let val_matcher: Box<dyn Matcher<ActualT = serde_json::Value>> = Box::new($val);
                map.insert(stringify!($key).to_owned(), val_matcher);
            })*

            $crate::matchers::JsonObjMatcher { expect: map }
        }
    };
}

pub struct JsonObjMatcher {
    pub expect: HashMap<String, Box<dyn Matcher<ActualT = serde_json::Value>>>,
}

impl Matcher for JsonObjMatcher {
    type ActualT = serde_json::Value;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
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

    fn describe(&self, matcher_result: MatcherResult) -> String {
        let mut result = match matcher_result {
            MatcherResult::Match => "is",
            MatcherResult::NoMatch => "is not",
        }
        .to_owned();

        if self.expect.is_empty() {
            return format!("{} an empty JSON object", result);
        }

        let mut first = true;
        result.push_str(" a JSON object with key(s): ");
        for (key, val) in self.expect.iter() {
            if first {
                first = false
            } else {
                result.push_str(", ");
            }
            result.push_str(&format!(
                "(\"{}\" with value that {})",
                key,
                val.describe(matcher_result),
            ));
        }

        result
    }
}

pub fn json_string(
    matcher: impl Matcher<ActualT = String> + 'static,
) -> impl Matcher<ActualT = serde_json::Value> {
    pat!(serde_json::Value::String(matcher))
}
pub fn json_number(
    matcher: impl Matcher<ActualT = serde_json::Number> + 'static,
) -> impl Matcher<ActualT = serde_json::Value> {
    pat!(serde_json::Value::Number(matcher))
}

pub struct UuidStrMatcher<ExpectedM, ActualT>
where
    ExpectedM: Matcher<ActualT = Uuid>,
    ActualT: AsRef<str>,
{
    expect: ExpectedM,
    phantom: PhantomData<ActualT>,
}

impl<ExpectedM, ActualT> Matcher for UuidStrMatcher<ExpectedM, ActualT>
where
    ExpectedM: Matcher<ActualT = Uuid>,
    ActualT: AsRef<str> + std::fmt::Debug,
{
    type ActualT = ActualT;

    fn matches(&self, actual: &ActualT) -> MatcherResult {
        let Ok(uuid) = Uuid::parse_str(actual.as_ref()) else {
            return MatcherResult::NoMatch;
        };

        self.expect.matches(&uuid)
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        let prefix = match matcher_result {
            MatcherResult::Match => "is",
            MatcherResult::NoMatch => "is not",
        };
        format!(
            "{} a UUID string that {}",
            prefix,
            self.expect.describe(matcher_result)
        )
    }
}

pub fn uuid_str<ExpectedM, ActualT>(matcher: ExpectedM) -> UuidStrMatcher<ExpectedM, ActualT>
where
    ExpectedM: Matcher<ActualT = Uuid>,
    ActualT: AsRef<str>,
{
    UuidStrMatcher {
        expect: matcher,
        phantom: PhantomData,
    }
}
