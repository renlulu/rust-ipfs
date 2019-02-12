extern crate multiaddr;

use multiaddr::Multiaddr;
use multihash::Hash;

pub struct Peer {
    pub id: Hash,
    pub addressed: Vec<Multiaddr>,
}

impl Peer {
    fn add_address(&mut self, address: Multiaddr) {
        self.addressed.push(address)
    }
}

pub struct PeerBook {}


fn main() {
    println!("Hello, world!");
}
