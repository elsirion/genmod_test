use api::module::{FederationModule, InputMeta, OutPoint, PeerId};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub struct Mint;

#[async_trait(?Send)]
impl FederationModule for Mint {
    type Error = MintError;
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
        Err(MintError("not implemented".into()))
    }

    fn validate_output(&self, _output: &Self::TxOutput) -> Result<u64, Self::Error> {
        todo!()
    }

    fn apply_output<'a>(
        &'a self,
        _output: &'a Self::TxOutput,
        _out_point: OutPoint,
    ) -> Result<u64, Self::Error> {
        println!("processed mint output");
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
pub struct MintError(String);

impl Error for MintError {}

impl Display for MintError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
