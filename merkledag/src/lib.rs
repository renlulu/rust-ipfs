#![feature(uniform_paths)]
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate serde;

use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};

extern crate multihash;

use multihash::Hash;

pub mod node;

use node::PBNode;
use node::PBLink;
use protobuf::repeated::RepeatedField;

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
}

impl Default for Node {
    fn default() -> Self {
        Node {
            links: vec![],
            data: vec![],
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
