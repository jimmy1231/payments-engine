use std::error::Error;

#[inline(always)]
fn trunc4(value: f64) -> f64 {
    (value * 10_000.0).trunc() / 10_000.0
}

#[derive(Default, Debug)]
pub struct Account {
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub locked: bool,
}

impl Account {
    pub fn new(client: u16) -> Self {
        Account {
            client,
            ..Default::default()
        }
    }

    pub fn total(&self) -> f64 {
        trunc4(self.available + self.held)
    }

    pub fn deposit(&mut self, amount: f64) -> Result<(), Box<dyn Error>> {
        if self.locked {
            return Err("Account is locked".into());
        }
        self.available = trunc4(self.available + amount);
        Ok(())
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<(), Box<dyn Error>> {
        if self.locked {
            return Err("Account is locked".into());
        }
        if amount > self.available {
            return Err("Insufficient funds".into());
        }
        self.available = trunc4(self.available - amount);
        Ok(())
    }

    pub fn hold(&mut self, amount: f64) -> Result<(), Box<dyn Error>> {
        self.held += trunc4(amount);
        self.available = trunc4(self.available - amount);
        Ok(())
    }

    pub fn release(&mut self, amount: f64) -> Result<(), Box<dyn Error>> {
        if amount > self.held {
            return Err("Insufficient held funds".into());
        }
        self.held = trunc4(self.held - amount);
        self.available = trunc4(self.available + amount);
        Ok(())
    }

    pub fn withdraw_from_hold(&mut self, amount: f64) -> Result<(), Box<dyn Error>> {
        if amount > self.held {
            return Err("Insufficient held funds".into());
        }
        self.held = trunc4(self.held - amount);
        Ok(())
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_account() -> Account {
        Account::new(1)
    }

    #[test]
    fn deposit_increases_available_and_truncates() {
        let mut acct = make_account();
        acct.deposit(1.23456).unwrap();
        assert_eq!(acct.available, 1.2345);
    }

    #[test]
    fn withdraw_decreases_available_and_errors_when_insufficient() {
        let mut acct = make_account();
        acct.deposit(5.0).unwrap();
        acct.withdraw(1.25).unwrap();
        assert_eq!(acct.available, 3.75);
        assert!(acct.withdraw(10.0).is_err());
    }

    #[test]
    fn hold_and_release_move_funds_between_available_and_held() {
        let mut acct = make_account();
        acct.deposit(10.0).unwrap();
        acct.hold(6.0).unwrap();
        assert_eq!(acct.available, 4.0);
        assert_eq!(acct.held, 6.0);

        acct.release(2.0).unwrap();
        assert_eq!(acct.available, 6.0);
        assert_eq!(acct.held, 4.0);
    }

    #[test]
    fn withdraw_from_hold_reduces_held_only() {
        let mut acct = make_account();
        acct.deposit(10.0).unwrap();
        acct.hold(5.0).unwrap();
        acct.withdraw_from_hold(3.0).unwrap();
        assert_eq!(acct.held, 2.0);
        assert_eq!(acct.available, 5.0);
    }

    #[test]
    fn locked_account_fails_deposit_and_withdraw() {
        let mut acct = make_account();
        acct.lock();
        assert!(acct.deposit(1.0).is_err());
        assert!(acct.withdraw(1.0).is_err());
    }
}
