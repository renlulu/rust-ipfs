extern crate peer;
extern crate storage;
extern crate netmux;
extern crate routing;
extern crate bitswap;
extern crate blocks;
extern crate path;

use peer::{Peer, PeerBook};
use storage::Storage;
use netmux::Netux;
use routing::Routing;
use bitswap::BitSwap;
use blocks::BlockService;
use path::{NameSystem, PathResolver};


pub struct IPFSNode {
    // the local node's identity
    pub identity: Peer,

    // the book of other nodes (hashtable)
    pub peer_book: PeerBook,

    // the local database
    pub storage: Storage,

    // the network message stream
    pub network: Netux,

    // the routing system, aka ipfs-dht
    pub routing: Routing,

    pub bitswap: BitSwap,

    // the block service, get/add blocks
    pub blocks: BlockService,

    // the path resolution system
    pub resolver: PathResolver,

    // the name system, resolves paths to hashes
    pub name_system: NameSystem,

}

fn main() {
    println!("Hello, world!");
}
