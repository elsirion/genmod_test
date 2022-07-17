use crate::genum::{Enum, EnumNil};
use crate::gmap::GMap;
use crate::module::{InputMeta, OutPoint};
use crate::{FederationModule, ToEnumType};
use async_trait::async_trait;
use hlist::Cons;
use std::marker::PhantomData;

#[async_trait(?Send)]
pub trait ProcessMapped: GMap + 'static {
    type ExtraArgs;
    type Output<M: FederationModule>;

    async fn process_item<M: FederationModule>(
        module: &M,
        item: &Self::Mapped<M>,
        args: Self::ExtraArgs,
    ) -> Self::Output<M>;
}

pub struct TxOutput;

impl GMap for TxOutput {
    type Mapped<I: FederationModule> = I::TxOutput;
}

/// Define how an associated type of a module is to be processed by a module (input, output, …)
#[async_trait(?Send)]
impl ProcessMapped for TxOutput {
    type ExtraArgs = OutPoint;
    type Output<M: FederationModule> = Result<u64, M::Error>;

    async fn process_item<M: FederationModule>(
        module: &M,
        item: &Self::Mapped<M>,
        out_point: Self::ExtraArgs,
    ) -> Self::Output<M> {
        module.apply_output(item, out_point)
    }
}

pub struct TxInput;

impl GMap for TxInput {
    type Mapped<I: FederationModule> = I::TxInput;
}

pub struct ModuleResult<T>(PhantomData<T>);

impl<T> GMap for ModuleResult<T> {
    type Mapped<I: FederationModule> = Result<T, I::Error>;
}

#[async_trait(?Send)]
impl ProcessMapped for TxInput {
    type ExtraArgs = ();
    type Output<M: FederationModule> = Result<InputMeta, M::Error>;

    async fn process_item<M: FederationModule>(
        module: &M,
        item: &Self::Mapped<M>,
        _args: Self::ExtraArgs,
    ) -> Self::Output<M> {
        module.apply_input(item)
    }
}

#[async_trait(?Send)]
pub trait Process<M: ProcessMapped>: ToEnumType {
    type OutputEnum: 'static;

    async fn process(&self, item_enum: Self::EnumType<M>, args: M::ExtraArgs) -> Self::OutputEnum;
}

#[async_trait(?Send)]
impl<M: ProcessMapped> Process<M> for hlist::Nil {
    type OutputEnum = EnumNil;

    async fn process(
        &self,
        _item_enum: Self::EnumType<M>,
        _args: M::ExtraArgs,
    ) -> Self::OutputEnum {
        panic!("We shouldn't reach this point if the enum had any variant")
    }
}

#[async_trait(?Send)]
impl<M: ProcessMapped, P, N> Process<M> for Cons<P, N>
where
    P: FederationModule + 'static,
    N: Process<M>,
{
    type OutputEnum = Enum<M::Output<P>, N::OutputEnum>;

    async fn process(&self, item_enum: Self::EnumType<M>, args: M::ExtraArgs) -> Self::OutputEnum {
        match item_enum {
            Enum::Payload(item) => Enum::Payload(M::process_item(&self.0, &item, args).await),
            Enum::Next(next) => Enum::Next(self.1.process(next, args).await),
        }
    }
}
