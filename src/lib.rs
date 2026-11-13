//! Clever Cloud CSI: Kubernetes protocol endpoints with the backend as the storage authority.
pub mod csi {
    #![allow(clippy::large_enum_variant)]
    tonic::include_proto!("csi.v1");
}

pub mod backend;
pub mod controller;
pub mod identity;
pub mod node;
pub mod server;
