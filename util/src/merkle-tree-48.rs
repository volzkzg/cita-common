// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// Acknowledgement: Based on the implementation of Boyu's merklehash.rs

use H256;
use hashable::{Hashable, HASH_NULL_RLP};
use rlp::RlpStream;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    nodes: Vec<H256>;
    votes_size: i32;
}

impl MerkleTree {
    pub fn generate_merkle_tree(votes: Vec<H256>, seq_st: i32) -> Self {
        let votes_size = votes.len() as i32;
        let nodes = match _votes_size {
            0 => vec![HASH_NULL_RLP],
            1 => votes,
            _ => {
                let nodes_size = get_nodes_size(votes_size);
                let internal_nodes_size = nodes_size - votes_size;
                let mut nodes = vec![H256::default(), nodes_size];
                for i in internal_nodes_size .. nodes_size {
                    nodes[i] = votes[i - internal_nodes_size];
                }
                for (i = internal_nodes_size - 1; i >=0; --i) {
                    nodes[i] = merge(&nodes[(i << 1) + 1],
                                     &nodes[(i << 1) + 2]);
                }
                nodes
            }
        }

        MerkleTree {
            nodes: nodes,
            input_size: input_size,
        }
    }
}

fn get_nodes_size(votes_size: i32) -> i32 {
    2 * votes_size - 1
}

fn merge(left &H256, right: &H256) {
    let mut stream = RlpStream::new();
    stream.append(left);
    stream.append(right);
    stream.out.crypt_hash()
}
