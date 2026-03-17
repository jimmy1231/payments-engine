use std::collections::HashMap;

use crate::{account::Account, ledger::Ledger};

pub struct ClientManager {
    accounts: HashMap<u16, Account>,
    ledgers: HashMap<u16, Ledger>,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            accounts: HashMap::new(),
            ledgers: HashMap::new(),
        }
    }

    pub fn get(&mut self, client: u16) -> (&mut Account, &mut Ledger) {
        let account = self.accounts.entry(client).or_insert_with(|| Account::new(client));
        let ledger = self.ledgers.entry(client).or_insert_with(Ledger::new);
        (account, ledger)
    }

    pub fn get_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }
}
