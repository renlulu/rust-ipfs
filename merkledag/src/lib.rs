#![feature(uniform_paths)]

extern crate multihash;

use multihash::Hash;

pub mod node;

use node::PBNode;
use node::PBLink;
use protobuf::repeated::RepeatedField;

pub struct Link {
    // utf string name, should be unique per object
    pub name: String,
    pub size: u64,
    pub hash: Vec<u8>,
}

// A node in the IPFS Merkle DAG.
// nodes have opaque data and a set of navigable links.
pub struct Node {
    pub links: RepeatedField<Link>,
    pub data: Vec<u8>,
}

impl Node {
    fn get_pb_node(&self) -> PBNode {
        let mut pbn = PBNode::new();
        let mut links = RepeatedField::default();
        for index in 0..self.links.len() {
            let l: &Link = self.links.get(index).unwrap();
            //todo
            let mut pb_link = PBLink::new();
            pb_link.set_Name(l.name.clone());
            pb_link.set_Hash(l.hash.clone());
            pb_link.set_Tsize(l.size)
        };
        pbn.set_Links(links);
        pbn.set_Data(self.data.clone());
        pbn
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
