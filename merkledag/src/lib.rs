extern crate multihash;

use multihash::Hash;

pub struct Link {
    // utf string name, should be unique per object
    pub name: String,
    pub size: u64,
    pub hash: Hash,
}

// A node in the IPFS Merkle DAG.
// nodes have opaque data and a set of navigable links.
pub struct Node {
    pub links: Vec<Link>,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
