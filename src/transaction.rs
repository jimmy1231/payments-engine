use serde::{Deserialize};
use std::error::Error;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransactionState {
    Normal,
    Disputed,
    Chargeback,
}

impl Default for TransactionState {
    fn default() -> Self {
        TransactionState::Normal
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub r#type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: f64,

    #[serde(skip_deserializing)]
    pub state: TransactionState,
}

impl Transaction {
    pub fn set_state(&mut self, state: TransactionState) {
        self.state = state;
    }

    pub fn reset_state(&mut self) {
        self.state = TransactionState::Normal;
    }

    pub fn assert_state(&self, expected_state: TransactionState) -> Result<(), Box<dyn Error>> {
        if self.state != expected_state {
            return Err(format!("Expected transaction state {:?}, but found {:?}", expected_state, self.state).into());
        }
        Ok(())
    }
}