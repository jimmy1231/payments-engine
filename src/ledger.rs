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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::{Transaction, TransactionState};

    fn make_tx(tx_id: u32) -> Transaction {
        Transaction {
            r#type: "deposit".to_string(),
            client: 1,
            tx: tx_id,
            amount: 1.0,
            state: TransactionState::Normal,
        }
    }

    #[test]
    fn append_stores_transaction_and_prevents_duplicates() {
        let mut ledger = Ledger::new();
        let tx = make_tx(1);
        assert!(ledger.append(tx.clone()).is_ok());
        assert!(ledger.get_transaction(1).is_some());
        assert!(ledger.append(tx).is_err());
    }
}
