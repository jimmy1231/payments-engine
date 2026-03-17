use std::error::Error;

use crate::{account::Account, ledger::Ledger, transaction::{Transaction, TransactionState}};

pub fn process_transaction(
    ledger: &mut Ledger,
    account: &mut Account,
    transaction: Transaction,
) -> Result<(), Box<dyn Error>> {
    if account.locked {
        return Err("Account is locked".into());
    }

    let transaction_type = &transaction.r#type;
    let amount = transaction.amount;
    let transaction_log = transaction.clone();

    let result = match transaction_type.as_str() {
        "deposit" => {
            account.deposit(amount)?;
            ledger.append(transaction)
        }
        "withdrawal" => {
            account.withdraw(amount)?;
            ledger.append(transaction)
        }
        "dispute" => {
            // mark transaction as disputed, then hold the amount in the account
            let tx = ledger
                .get_transaction(transaction.tx)
                .ok_or("Transaction not found")?;
            tx.assert_state(TransactionState::Normal)?;
            tx.set_state(TransactionState::Disputed);
            account.hold(tx.amount)
        }
        "resolve" => {
            // reset transaction state, and release held amount
            let tx = ledger
                .get_transaction(transaction.tx)
                .ok_or("Transaction not found")?;
            tx.assert_state(TransactionState::Disputed)?;
            tx.reset_state();
            account.release(tx.amount)
        }
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
        }
        _ => Err("Unknown transaction type".into()),
    };
    
    ledger.log(transaction_log, result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tx(tx_type: &str, client: u16, tx: u32, amount: f64) -> Transaction {
        Transaction {
            r#type: tx_type.to_string(),
            client,
            tx,
            amount,
            state: TransactionState::Normal,
        }
    }

    #[test]
    fn deposit_adds_to_available_and_stores_tx() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);
        let tx = make_tx("deposit", 1, 1, 10.0);

        process_transaction(&mut ledger, &mut account, tx).unwrap();

        assert_eq!(account.available, 10.0);
        assert!(ledger.get_transaction(1).is_some());
    }

    #[test]
    fn withdrawal_subtracts_from_available_and_stores_tx() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);

        let deposit_tx = make_tx("deposit", 1, 1, 10.0);
        process_transaction(&mut ledger, &mut account, deposit_tx).unwrap();

        let tx = make_tx("withdrawal", 1, 2, 4.5);
        process_transaction(&mut ledger, &mut account, tx).unwrap();

        assert_eq!(account.available, 5.5);
        assert!(ledger.get_transaction(2).is_some());
    }

    #[test]
    fn dispute_moves_amount_to_held_and_marks_tx_disputed() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);

        let deposit_tx = make_tx("deposit", 1, 1, 10.0);
        process_transaction(&mut ledger, &mut account, deposit_tx).unwrap();

        let dispute_tx = make_tx("dispute", 1, 1, 0.0);
        process_transaction(&mut ledger, &mut account, dispute_tx).unwrap();

        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 10.0);
        assert_eq!(ledger.get_transaction(1).unwrap().state, TransactionState::Disputed);
    }

    #[test]
    fn resolve_returns_held_amount_to_available_and_marks_tx_normal() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);

        let deposit_tx = make_tx("deposit", 1, 1, 10.0);
        process_transaction(&mut ledger, &mut account, deposit_tx).unwrap();

        let dispute_tx = make_tx("dispute", 1, 1, 0.0);
        process_transaction(&mut ledger, &mut account, dispute_tx).unwrap();

        let resolve_tx = make_tx("resolve", 1, 1, 0.0);
        process_transaction(&mut ledger, &mut account, resolve_tx).unwrap();

        assert_eq!(account.available, 10.0);
        assert_eq!(account.held, 0.0);
        assert_eq!(ledger.get_transaction(1).unwrap().state, TransactionState::Normal);
    }

    #[test]
    fn chargeback_withdraws_held_and_locks_account() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);

        let deposit_tx = make_tx("deposit", 1, 1, 10.0);
        process_transaction(&mut ledger, &mut account, deposit_tx).unwrap();

        let dispute_tx = make_tx("dispute", 1, 1, 0.0);
        process_transaction(&mut ledger, &mut account, dispute_tx).unwrap();

        let chargeback_tx = make_tx("chargeback", 1, 1, 0.0);
        process_transaction(&mut ledger, &mut account, chargeback_tx).unwrap();

        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 0.0);
        assert!(account.locked);
        assert_eq!(ledger.get_transaction(1).unwrap().state, TransactionState::Chargeback);
    }

    #[test]
    fn unknown_type_does_not_affect_account_or_ledger() {
        let mut ledger = Ledger::new();
        let mut account = Account::new(1);

        let tx = make_tx("unknown", 1, 1, 1.0);
        let result = process_transaction(&mut ledger, &mut account, tx);

        // process_transaction always returns Ok after logging; the failure is captured in the log
        assert!(result.is_ok());
        assert_eq!(account.available, 0.0);
        assert_eq!(account.held, 0.0);
        assert!(ledger.get_transaction(1).is_none());
    }
}
