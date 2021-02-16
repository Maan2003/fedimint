#![feature(async_closure)]
#![feature(iterator_fold_self)]

/// Configuration structs for server and client
pub mod config;

/// The actual implementation of the federated mint
pub mod consensus;

/// The implementation of mint primitives
pub mod mint;

/// Networking for mint-to-mint and client-to-mint communiccation
pub mod net;

/// Client helper code to generate requests
pub mod client;
