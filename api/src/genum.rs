extern crate core;

use std::fmt::Debug;
use std::str::FromStr;

/// Enum that gets constructed through the type system.
/// 
/// Note that even if the concrete varaint might be `Enum::Payload(Foo)`, the required type might
/// still be `Enum<Foo, Enum<Bar, EnumNil>>`. 
#[derive(Debug)]
pub enum Enum<P, N> {
    Payload(P),
    Next(N),
}

#[derive(Debug)]
pub struct EnumNil;

/// Could be useful for serialization?
pub trait EnumIdx {
    const IDX: usize;

    fn idx(&self) -> usize;

    fn from_idx(idx: usize, val: &str) -> Self;
}

impl<T, O> EnumIdx for Enum<T, O>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    O: EnumIdx,
{
    const IDX: usize = O::IDX + 1;

    fn idx(&self) -> usize {
        Self::IDX
    }

    fn from_idx(idx: usize, val: &str) -> Self {
        if idx == Self::IDX {
            Enum::Payload(FromStr::from_str(val).unwrap())
        } else {
            Enum::Next(O::from_idx(idx, val))
        }
    }
}

impl EnumIdx for EnumNil {
    const IDX: usize = 0;

    fn idx(&self) -> usize {
        panic!()
    }

    fn from_idx(_idx: usize, _val: &str) -> Self {
        EnumNil
    }
}
