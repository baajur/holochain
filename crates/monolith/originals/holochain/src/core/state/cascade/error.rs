use monolith::holochain::core::workflow::produce_dht_ops_workflow::dht_op_light::error::DhtOpConvertError;
use monolith::holochain::core::SourceChainError;
use holo_hash::AnyDhtHash;
use monolith::holochain_p2p::HolochainP2pError;
use holochain_serialized_bytes::SerializedBytesError;
use monolith::holochain_state::error::DatabaseError;
use monolith::holochain_types::dht_op::error::DhtOpError;
use monolith::holochain_types::element::error::ElementGroupError;
use monolith::holochain_zome_types::header::conversions::WrongHeaderError;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub enum CascadeError {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    ElementGroupError(#[from] ElementGroupError),

    #[error(transparent)]
    DhtOpConvertError(#[from] DhtOpConvertError),

    #[error(transparent)]
    DhtOpError(#[from] DhtOpError),

    #[error("Got an invalid response from an authority for the request hash: {0:?}")]
    InvalidResponse(AnyDhtHash),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    SourceChainError(#[from] SourceChainError),

    #[error(transparent)]
    NetworkError(#[from] HolochainP2pError),

    #[error(transparent)]
    SerializedBytesError(#[from] SerializedBytesError),

    #[error(transparent)]
    WrongHeaderError(#[from] WrongHeaderError),
}

pub type CascadeResult<T> = Result<T, CascadeError>;