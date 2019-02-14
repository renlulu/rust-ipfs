#![feature(uniform_paths)]
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;
extern crate cid;


extern crate multihash;

use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use multihash::Hash;

pub mod node;

use node::PBNode;
use node::PBLink;
use protobuf::repeated::RepeatedField;
use cid::Cid;
use cid::Codec;
use cid::Version;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Link {
    // utf string name, should be unique per object
    pub name: String,
    pub size: u64,
    pub hash: Vec<u8>,
}

// A node in the IPFS Merkle DAG.
// nodes have opaque data and a set of navigable links.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Node {
    pub links: Vec<Link>,
    pub data: Vec<u8>,
    pub encoded: Vec<u8>,
    pub cached: cid::Cid,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            links: vec![],
            data: vec![],
            encoded: vec![],
            cached: Cid::new(Codec::DagProtobuf, Version::V1, &multihash::encode(multihash::Hash::SHA2256, b"").unwrap())
        }
    }
}

impl Node {
    pub fn get_pb_node(&self) -> PBNode {
        let mut pbn = PBNode::new();
        let mut links = RepeatedField::default();
        for index in 0..self.links.len() {
            let l: &Link = self.links.get(index).unwrap();
            let mut pb_link = PBLink::new();
            pb_link.set_Name(l.name.clone());
            pb_link.set_Hash(l.hash.clone());
            pb_link.set_Tsize(l.size)
        };
        pbn.set_Links(links);
        pbn.set_Data(self.data.clone());
        pbn
    }

    pub fn marshal(&self) -> Vec<u8> {
        let encoded: Vec<u8> = serialize(self).unwrap();
        encoded
    }
}

#[cfg(test)]
mod tests {
    use crate::Node;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_marshal() {
        let node = Node::default();
        let bytes = node.marshal();
        println!("{:?}", bytes);
    }
}
