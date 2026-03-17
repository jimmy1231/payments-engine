mod account;
mod ledger;
mod transaction;
mod client_manager;

use std::{error::Error, env, io};

use crate::account::Account;
use crate::ledger::Ledger;
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

fn process_transaction(ledger: &mut Ledger, account: &mut Account, transaction: Transaction) -> Result<(), Box<dyn Error>> {
    if account.locked {
        return Err("Account is locked".into());
    }
    
    let transaction_type = &transaction.r#type;
    let amount = transaction.amount;

    let result = match transaction_type.as_str() {
        "deposit" => {
            account.deposit(amount)?;
            ledger.append(transaction.clone())
        },
        "withdrawal" => {
            account.withdraw(amount)?;
            ledger.append(transaction.clone())
        },
        "dispute" => {
            // mark transaction as disputed, then hold the amount in the account
            let tx = ledger
                .get_transaction(transaction.tx)
                .ok_or("Transaction not found")?;
            tx.assert_state(TransactionState::Normal)?;
            tx.set_state(TransactionState::Disputed);
            account.hold(tx.amount)
        },
        "resolve" => {
            // reset transaction state, and release held amount
            let tx = ledger
                .get_transaction(transaction.tx)
                .ok_or("Transaction not found")?;
            tx.assert_state(TransactionState::Disputed)?;
            tx.reset_state();
            account.release(tx.amount)
        },
        "chargeback" => {
            // mark transaction as chargeback, withdraw held amount, and lock account
            let tx = ledger
                .get_transaction(transaction.tx)
                .ok_or("Transaction not found")?;
            tx.assert_state(TransactionState::Disputed)?;
            tx.set_state(TransactionState::Chargeback);
            account.withdraw_from_hold(tx.amount)?;
            account.lock();
            Ok(())
        },
        _ => Err("Unknown transaction type".into()),
    };

    // log all transactions
    ledger.log(transaction, result);
    Ok(())
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
        if let Err(_) = process_transaction(ledger, account, transaction) {
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