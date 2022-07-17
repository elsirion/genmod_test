use api::module::{FederationModule, InputMeta, OutPoint, PeerId};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct Wallet;

#[async_trait(?Send)]
impl FederationModule for Wallet {
    type Error = WalletError;
    type TxInput = ();
    type TxOutput = ();
    type TxOutputOutcome = ();
    type ConsensusItem = ();

    async fn await_consensus_proposal<'a>(&'a self) {
        todo!()
    }

    async fn consensus_proposal<'a>(&'a self) -> Vec<Self::ConsensusItem> {
        todo!()
    }

    async fn begin_consensus_epoch<'a>(
        &'a self,
        _consensus_items: Vec<(PeerId, Self::ConsensusItem)>,
    ) {
        todo!()
    }

    fn validate_input(&self, _input: &Self::TxInput) -> Result<InputMeta, Self::Error> {
        todo!()
    }

    fn apply_input(&self, _input: &Self::TxInput) -> Result<InputMeta, Self::Error> {
        Err(WalletError("FOOOOOO".into()))
    }

    fn validate_output(&self, _output: &Self::TxOutput) -> Result<u64, Self::Error> {
        todo!()
    }

    fn apply_output<'a>(
        &'a self,
        _output: &'a Self::TxOutput,
        _out_point: OutPoint,
    ) -> Result<u64, Self::Error> {
        println!("processed wallet output");
        Ok(0)
    }

    async fn end_consensus_epoch<'a>(&'a self) -> Vec<PeerId> {
        todo!()
    }

    fn output_status(&self, _out_point: OutPoint) -> Option<Self::TxOutputOutcome> {
        todo!()
    }

    fn audit(&self, _audit: &mut String) {
        todo!()
    }

    fn api_base_name(&self) -> &'static str {
        todo!()
    }

    fn api_endpoints(&self) -> &'static [&'static str] {
        todo!()
    }
}

#[derive(Debug)]
pub struct WalletError(String);

impl Error for WalletError {}

impl Display for WalletError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
