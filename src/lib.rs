//! Nakamoto is a high-assurance Bitcoin light-client library.
//!
//! The project is broken down into the following crates:
//!
//! * [`client`]: the core light-client library
//! * [`p2p`]: the protocol implementation
//! * [`chain`]: the block store and fork selection logic
//! * [`common`]: common functionality used by all crates
//!
//! The [`client`] crate is intended to be the entry point for most users of the
//! library, and is a good place to start, to see how everything fits together.
//!
//! ```no_run
//! use std::{net, thread};
//!
//! use nakamoto::client::{Publisher, Client, Config, Network, Services};
//! use nakamoto::client::error::Error;
//! use nakamoto::client::handle::Handle as _;
//!
//! /// The network reactor we're going to use.
//! type Reactor = nakamoto::net::poll::Reactor<net::TcpStream, Publisher>;
//!
//! /// Run the light-client.
//! fn main() -> Result<(), Error> {
//!     let cfg = Config {
//!         network: Network::Testnet,
//!         ..Config::default()
//!     };
//!     // Create a client using the above network reactor.
//!     let client = Client::<Reactor>::new(cfg)?;
//!     let handle = client.handle();
//!
//!     // Run the client on a different thread, to not block the main thread.
//!     thread::spawn(|| client.run().unwrap());
//!
//!     // Wait for the client to be connected to a peer.
//!     handle.wait_for_peers(1, Services::default())?;
//!
//!     // Ask the client to terminate.
//!     handle.shutdown()?;
//!
//!     Ok(())
//! }
//! ```
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

#[cfg(feature = "nakamoto-chain")]
pub use nakamoto_chain as chain;
#[cfg(feature = "nakamoto-client")]
pub use nakamoto_client as client;
#[cfg(feature = "nakamoto-common")]
pub use nakamoto_common as common;
#[cfg(feature = "nakamoto-node")]
pub use nakamoto_node as node;
#[cfg(feature = "nakamoto-p2p")]
pub use nakamoto_p2p as p2p;
#[cfg(feature = "nakamoto-wallet")]
pub use nakamoto_wallet as wallet;

#[cfg(test)]
#[cfg(feature = "nakamoto-test")]
pub use nakamoto_test as test;

pub mod net {
    #[cfg(feature = "nakamoto-net-poll")]
    pub use nakamoto_net_poll as poll;
}
