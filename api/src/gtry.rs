use crate::genum::{Enum, EnumNil};
use std::error::Error;

pub trait MapEnum {}

/// Makes an enum of results into a result whose error type is an enum over all module errors.
///
/// Basically `Enum<Result<T, FooError>, Result<T, BarError>> -> Result<T, Enum<FooError, BarError>>`
pub trait GTry<T> {
    type ReturnError;

    fn generic_try(self) -> Result<T, Self::ReturnError>;
}

impl<T> GTry<T> for EnumNil {
    type ReturnError = EnumNil;

    fn generic_try(self) -> Result<T, Self::ReturnError> {
        panic!("We should never reach this point")
    }
}

impl<T, N, E> GTry<T> for Enum<Result<T, E>, N>
where
    N: GTry<T>,
{
    type ReturnError = Enum<E, N::ReturnError>;

    fn generic_try(self) -> Result<T, Self::ReturnError> {
        match self {
            Enum::Payload(Ok(val)) => Ok(val),
            Enum::Payload(Err(e)) => Err(Enum::Payload(e)),
            Enum::Next(next) => {
                let res = next.generic_try();
                match res {
                    Ok(val) => Ok(val),
                    Err(e) => Err(Enum::Next(e)),
                }
            }
        }
    }
}

/// Generic enum errors are annoying, let's use anyhow wherever we can
pub trait GTryAny<T> {
    fn to_any(self) -> Result<T, anyhow::Error>;
}

impl<T, E> GTryAny<T> for Result<T, E>
where
    E: GToAnyhow,
{
    fn to_any(self) -> Result<T, anyhow::Error> {
        self.map_err(GToAnyhow::to_anyhow)
    }
}

pub trait GToAnyhow {
    fn to_anyhow(self) -> anyhow::Error;
}

impl GToAnyhow for EnumNil {
    fn to_anyhow(self) -> anyhow::Error {
        panic!("The enum was empty")
    }
}

impl<P, N> GToAnyhow for Enum<P, N>
where
    P: Error + Send + Sync + 'static,
    N: GToAnyhow,
{
    fn to_anyhow(self) -> anyhow::Error {
        match self {
            Enum::Payload(e) => anyhow::Error::new(e),
            Enum::Next(n) => n.to_anyhow(),
        }
    }
}
