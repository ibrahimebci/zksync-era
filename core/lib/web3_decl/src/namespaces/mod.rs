pub mod debug;
pub mod en;
pub mod eth;
pub mod eth_subscribe;
pub mod net;
pub mod web3;
pub mod zks;

#[cfg(feature = "client")]
pub use self::{
    debug::DebugNamespaceClient, en::EnNamespaceClient, eth::EthNamespaceClient,
    net::NetNamespaceClient, web3::Web3NamespaceClient, zks::ZksNamespaceClient,
};
#[cfg(feature = "server")]
pub use self::{
    debug::DebugNamespaceServer, en::EnNamespaceServer, eth::EthNamespaceServer,
    net::NetNamespaceServer, web3::Web3NamespaceServer, zks::ZksNamespaceServer,
};
