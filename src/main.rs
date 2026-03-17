mod account;
mod ledger;
mod transaction;
mod client_manager;
mod engine;

use std::{error::Error, env, io};

use crate::client_manager::ClientManager;
use crate::transaction::{Transaction, TransactionState};

#[derive(serde::Serialize)]
struct AccountOutput {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

fn load_transactions(path: &String) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut transactions = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)?;
    for result in rdr.deserialize() {
        let mut record: Transaction = result?;
        record.set_state(TransactionState::Normal);
        transactions.push(record);
    }
    Ok(transactions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <transactions.csv>", args[0]);
        std::process::exit(1);
    }

    let transactions = load_transactions(&args[1])?;
    let mut manager = ClientManager::new();

    for transaction in transactions {
        let (account, ledger) = manager.get(transaction.client);
        if let Err(_) = engine::process_transaction(ledger, account, transaction) {
            // ignore error
        }
    }

    // output final account states as CSV
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for account in manager.get_accounts() {
        let output = AccountOutput {
            client: account.client,
            available: account.available,
            held: account.held,
            total: account.total(),
            locked: account.locked,
        };
        wtr.serialize(output)?;
    }
    wtr.flush()?;

    Ok(())
}