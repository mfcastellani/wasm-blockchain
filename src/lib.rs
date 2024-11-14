use wasm_bindgen::prelude::*;
use web_sys::console;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Transaction {
    id: u64,
    origin: String,
    destination: String,
    quantity: u64,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: Option<String>,
}

impl Block {
    fn new(id: u64, previous_hash: String) -> Self {
        Self {
            id,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            transactions: Vec::new(),
            previous_hash,
            hash: None,
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        if self.transactions.len() < 5 {
            self.transactions.push(transaction);
            if self.transactions.len() == 5 {
                self.hash = Some(self.calculate_hash());
            }
        }
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{:?}{}",
            self.id, self.timestamp, self.transactions, self.previous_hash
        );
        hasher.update(data);
        hex::encode(hasher.finalize())
    }
}

struct Blockchain {
    blocks: HashMap<u64, Block>,
    latest_block: Option<u64>,
}

#[allow(dead_code)]
impl Blockchain {
    fn new() -> Self {
        let mut genesis_block = Block::new(0, String::from("0"));
        genesis_block.hash = Some(genesis_block.calculate_hash());

        let mut blockchain = Self {
            blocks: HashMap::new(),
            latest_block: Some(0),
        };
        blockchain.blocks.insert(0, genesis_block);
        blockchain
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        if transactions.len() != 5 {
            panic!("A block must contain exactly 5 transactions.");
        }

        let latest_id = self.latest_block.unwrap();
        let previous_hash = self.blocks[&latest_id].hash.clone().unwrap();
        let mut block = Block::new(latest_id + 1, previous_hash);

        for transaction in transactions {
            block.add_transaction(transaction);
        }

        self.blocks.insert(block.id, block.clone());
        self.latest_block = Some(block.id);
    }

    fn get_block_by_id(&self, id: u64) -> Option<&Block> {
        self.blocks.get(&id)
    }

    fn validate_chain(&self) -> bool {
        let mut previous_hash = String::from("0");

        for id in 0..=self.latest_block.unwrap() {
            let block = &self.blocks[&id];

            if block.hash.is_none() || block.hash.as_ref().unwrap() != &block.calculate_hash() {
                return false;
            }

            if block.previous_hash != previous_hash {
                return false;
            }

            previous_hash = block.hash.clone().unwrap();
        }

        true
    }
}

#[wasm_bindgen]
pub fn validate()  {
    console::log_1(&"Starting blockchain".into());

    let mut blockchain = Blockchain::new();
    let mut transaction_id = 1;

    for block_id in 1..=20 {
        console::log_1(&"Adding transactions".into());
        let transactions: Vec<Transaction> = (0..5)
            .map(|_| {
                let transaction = Transaction {
                    id: transaction_id,
                    origin: format!("User{}", transaction_id),
                    destination: format!("User{}", transaction_id + 1),
                    quantity: transaction_id * 10,
                };
                transaction_id += 1;
                transaction
            })
            .collect();
        console::log_1(&"Adding block".into());
        blockchain.add_block(transactions);
        console::log_1(&format!("Added block with ID: {}", block_id).into());
    }

    if blockchain.validate_chain() {
        console::log_1(&"The blockchain is valid.".into());
    } else {
        console::log_1(&"The blockchain is not valid.".into());
    }
}
