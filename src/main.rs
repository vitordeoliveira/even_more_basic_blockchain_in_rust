use core::hash;
use std::thread::current;

#[derive(Debug)]
struct Block {
    index: u64,
    data: String,
    hash: String,
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Self {
        let data_copy = data.clone();
        Self {
            index,
            data,
            hash: format!("{prev_hash}_{data_copy}"),
        }
    }
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block {
            index: 0,
            data: "genesis".to_string(),
            hash: "genesis".to_string(),
        };

        Self {
            chain: vec![genesis],
        }
    }

    fn add_block(&mut self, data: String) {
        let index = self.chain.len();
        let prev_block = self.chain.get(index - 1).expect("blockchain error");

        let new_block = Block::new(index as u64, data, prev_block.hash.clone());

        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for b in 1..self.chain.len() {
            let current = self.chain.get(b).unwrap();
            let prev = self.chain.get(b - 1).unwrap();

            if current.hash != format!("{}_{}", prev.hash.clone(), current.data) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("vitor".to_string());
    blockchain.add_block("nicolas".to_string());
    blockchain.add_block("mariana".to_string());

    for b in &blockchain.chain {
        dbg!(b);
    }

    if blockchain.is_valid() {
        println!("Is a valid blockchain");
    } else {
        println!("Is a invalid blockchain");
    }
}
