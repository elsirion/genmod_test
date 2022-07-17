use crate::genum::{Enum, EnumNil};
use crate::module::FederationModule;

/// A zero size type that defines a mapping from a modules to a dependent/associated type thereof.
///
/// See also [`crate::process::ProcessMapped`] that extends this trait.
pub trait GMap {
    type Mapped<I: FederationModule>;
}

struct Ident;

impl GMap for Ident {
    type Mapped<I: FederationModule> = I;
}

/// Build an enum type containing a certain associated type of all modules
pub trait ToEnumType {
    type EnumType<M: GMap>;
}

impl ToEnumType for hlist::Nil {
    type EnumType<M: GMap> = EnumNil;
}

impl<P, N> ToEnumType for hlist::Cons<P, N>
where
    P: FederationModule,
    N: ToEnumType,
{
    type EnumType<M: GMap> = Enum<M::Mapped<P>, N::EnumType<M>>;
}

pub struct ModuleEror;

impl GMap for ModuleEror {
    type Mapped<I: FederationModule> = I::Error;
}
