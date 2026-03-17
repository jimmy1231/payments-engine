use std::collections::HashMap;
use std::error::Error;

use crate::transaction::Transaction;

pub struct Ledger {
    transactions: HashMap<u32, Transaction>,
    logs: Vec<(Transaction, Result<(), Box<dyn Error>>)>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            transactions: HashMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn get_transaction(&mut self, tx: u32) -> Option<&mut Transaction> {
        self.transactions.get_mut(&tx)
    }

    pub fn append(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>> {
        if self.transactions.contains_key(&transaction.tx) {
            return Err("Transaction ID already exists".into());
        }
        self.transactions.insert(transaction.tx, transaction);
        Ok(())
    }

    pub fn log(&mut self, transaction: Transaction, result: Result<(), Box<dyn Error>>) {
        self.logs.push((transaction, result));
    }
}
