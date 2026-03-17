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
