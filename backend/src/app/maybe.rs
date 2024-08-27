use sea_orm::{ActiveValue::NotSet, IntoActiveValue, Set};

#[derive(Clone, Debug, Default)]
pub(crate) enum Maybe<T> {
    Some(T),
    #[default]
    Undefined,
}

impl<T> Maybe<T> {
    pub(crate) fn into_option(self) -> Option<T> {
        match self {
            Maybe::Some(x) => Some(x),
            Maybe::Undefined => None,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_defined(&self) -> bool {
        matches!(self, Maybe::Some(_))
    }

    #[allow(dead_code)]
    pub(crate) fn is_undefined(&self) -> bool {
        matches!(self, Maybe::Undefined)
    }

    #[allow(dead_code)]
    pub(crate) fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<U> {
        match self {
            Maybe::Some(x) => Maybe::Some(f(x)),
            Maybe::Undefined => Maybe::Undefined,
        }
    }
}

impl<T> Maybe<Option<T>> {
    #[allow(dead_code)]
    pub(crate) fn map_nonnull<U, F: FnOnce(T) -> U>(self, f: F) -> Maybe<Option<U>> {
        match self {
            Maybe::Some(Some(x)) => Maybe::Some(Some(f(x))),
            Maybe::Some(None) => Maybe::Some(None),
            Maybe::Undefined => Maybe::Undefined,
        }
    }
}

impl<T, E> Maybe<Result<T, E>> {
    #[allow(dead_code)]
    pub(crate) fn transpose(self) -> Result<Maybe<T>, E> {
        match self {
            Maybe::Some(Ok(x)) => Ok(Maybe::Some(x)),
            Maybe::Some(Err(x)) => Err(x),
            Maybe::Undefined => Ok(Maybe::Undefined),
        }
    }
}

impl<T> From<Maybe<T>> for Option<T> {
    fn from(x: Maybe<T>) -> Self {
        x.into_option()
    }
}

impl<T> From<Option<T>> for Maybe<T> {
    fn from(x: Option<T>) -> Self {
        match x {
            Some(x) => Maybe::Some(x),
            None => Maybe::Undefined,
        }
    }
}

impl<T> IntoActiveValue<T> for Maybe<T>
where
    T: Into<sea_orm::Value>,
{
    fn into_active_value(self) -> sea_orm::ActiveValue<T> {
        match self {
            Maybe::Some(x) => Set(x),
            Maybe::Undefined => NotSet,
        }
    }
}
