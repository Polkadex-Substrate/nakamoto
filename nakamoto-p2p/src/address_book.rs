//! Address book. Keeps track of known peers.
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::net;
use std::path::Path;

use crate::peer::Network;

#[derive(Debug, PartialEq)]
pub struct AddressBook {
    pub addrs: Vec<net::SocketAddr>,
}

impl AddressBook {
    pub fn from<T: net::ToSocketAddrs + fmt::Debug>(seeds: &[T]) -> io::Result<Self> {
        let addrs = seeds
            .iter()
            .flat_map(|seed| match seed.to_socket_addrs() {
                Ok(addrs) => addrs.into_iter().map(Ok).collect(),
                Err(err) => vec![Err(err)],
            })
            .collect::<io::Result<_>>()?;

        Ok(Self { addrs })
    }

    pub fn bootstrap(network: Network) -> io::Result<Self> {
        match network {
            Network::Mainnet => {
                let seeds = network
                    .seeds()
                    .iter()
                    .map(|s| (*s, network.port()))
                    .collect::<Vec<_>>();

                AddressBook::from(&seeds)
            }
            _ => todo!(),
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        use std::io::BufReader;

        let file = File::open(path)?;
        let reader = BufReader::with_capacity(32, file);
        let mut addrs = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let addr = line.parse().unwrap();

            addrs.push(addr);
        }

        Ok(Self { addrs })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut f = File::create(path)?;

        for addr in self.addrs.iter() {
            writeln!(&mut f, "{}", addr)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_save_and_load() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("address-book");

        assert!(AddressBook::load(&path).is_err());

        let book = AddressBook::from(&[
            ("143.25.122.51", 8333),
            ("231.45.72.2", 8334),
            ("113.98.77.4", 8333),
        ])
        .unwrap();

        book.save(&path).unwrap();

        assert_eq!(AddressBook::load(&path).unwrap(), book);
    }
}
