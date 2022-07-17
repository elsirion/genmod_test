#![feature(generic_associated_types)]

extern crate core;

pub mod genum;
pub mod gmap;
pub mod gtry;
pub mod module;
pub mod process;

use crate::gmap::ToEnumType;
use crate::gtry::GTry;
use crate::module::{FederationModule, InputMeta, OutPoint};
use crate::process::{Process, TxInput, TxOutput};
use hlist::{Cons, HList, Nil as ListNil};

pub struct Federation<M> {
    modules: M,
}

/// Just a quick way to get types, e.g. to construct the tx type generically over the federation
pub trait FederationTypes {
    type TxOutput;
    type TxInput;
}

impl Federation<ListNil> {
    pub fn new() -> Federation<ListNil> {
        Federation { modules: ListNil }
    }
}

impl<M> Federation<M>
where
    M: HList,
{
    pub fn attach_module<T: FederationModule>(self, module: T) -> Federation<Cons<T, M>> {
        Federation {
            modules: self.modules.push(module),
        }
    }
}

impl<M> Federation<M>
where
    M: ToEnumType + Process<TxOutput> + Process<TxInput>,
    <M as Process<TxOutput>>::OutputEnum: GTry<u64>,
    <M as Process<TxInput>>::OutputEnum: GTry<InputMeta>,
{
    pub async fn process_output(
        &self,
        output: M::EnumType<TxOutput>,
        out_point: OutPoint,
    ) -> Result<u64, <<M as Process<TxOutput>>::OutputEnum as GTry<u64>>::ReturnError> {
        self.modules.process(output, out_point).await.generic_try()
    }

    pub async fn process_input(
        &self,
        input: M::EnumType<TxInput>,
    ) -> Result<InputMeta, <<M as Process<TxInput>>::OutputEnum as GTry<InputMeta>>::ReturnError>
    {
        self.modules.process(input, ()).await.generic_try()
    }
}

impl<M> FederationTypes for Federation<M>
where
    M: ToEnumType,
{
    type TxOutput = M::EnumType<TxOutput>;
    type TxInput = M::EnumType<TxInput>;
}
