fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

#[derive(Debug)]
struct Bank {
    name: String,
    accounts: HashMap<u32, Account>,
}
// ..
#[derive(Debug)]
struct User {
    name: String,
    user_id: u32,
}

#[derive(Debug)]
struct Account {
    account_id: u32,
    user: User,
    balance: f64,
    bank_name: String,
}

impl Bank {
    fn new(name: &str) -> Self {
        Bank {
            name: name.to_string(),
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, user: User, account_id: u32, initial_balance: f64) {
        let account = Account {
            account_id,
            user,
            balance: initial_balance,
            bank_name: self.name.clone(),
        };
        self.accounts.insert(account_id, account);
    }

    fn get_account_mut(&mut self, account_id: u32) -> Option<&mut Account> {
        self.accounts.get_mut(&account_id)
    }

    fn transfer_between_banks(&mut self, other_bank: &mut Bank, from_id: u32, to_id: u32, amount: f64) {
        let from_account = self.get_account_mut(from_id);
        let to_account = other_bank.get_account_mut(to_id);

        match (from_account, to_account) {
            (Some(from), Some(to)) => {
                if from.balance >= amount {
                    from.balance -= amount;
                    to.balance += amount;
                    println!("Transfer successful: {} -> {} ({} USD)", from.account_id, to.account_id, amount);
                } else {
                    println!("Transfer failed: Insufficient funds.");
                }
            }
            _ => println!("Transfer failed: One or both accounts not found."),
        }
    }
}

impl Account {
    fn transfer(&mut self, to: &mut Account, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            to.balance += amount;
            println!("Transfer successful: {} -> {} ({} USD)", self.account_id, to.account_id, amount);
        } else {
            println!("Transfer failed: Insufficient funds.");
        }
    }

    fn request_money(&mut self, from: &mut Account, amount: f64) {
        from.transfer(self, amount);
    }
}

fn main() {
    let mut bank_a = Bank::new("Bank A");
    let mut bank_b = Bank::new("Bank B");
    
    let user1 = User { name: "Alice".to_string(), user_id: 1 };
    let user2 = User { name: "Bob".to_string(), user_id: 2 };
    
    bank_a.create_account(user1, 101, 500.0);
    bank_b.create_account(user2, 202, 300.0);
    
    bank_a.transfer_between_banks(&mut bank_b, 101, 202, 200.0);
    
    if let (Some(acc1), Some(acc2)) = (bank_a.get_account_mut(101), bank_b.get_account_mut(202)) {
        acc2.request_money(acc1, 50.0);
        println!(
    "Final Balances: {} (ID: {}) - {} USD in {} | {} (ID: {}) - {} USD in {}",
    acc1.user.name, acc1.user.user_id, acc1.balance, acc1.bank_name,
    acc2.user.name, acc2.user.user_id, acc2.balance, acc2.bank_name
);

    }
}
