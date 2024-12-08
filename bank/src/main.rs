#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn deposit(&mut self, account_id: u32, amount: &i32) -> i32 {
        if *amount < 0 {
            println!("Invalid amount");
            return -1;
        }

        for account in &mut self.accounts {
            if account.id == account_id {
                account.balance += amount;
                return account.balance;
            }
        }

        println!("Invalid account id");
        -1
    }

    fn withdraw(&mut self, account_id: u32, amount: i32) -> i32 {
        for account in &mut self.accounts {
            if account.id == account_id {
                if account.balance < amount {
                    println!("Insufficient funds for {}", account.holder);
                    return -1;
                }

                account.balance -= amount;
                return account.balance;
            }
        }

        println!("Invalid account id");
        -1
    }

    fn summary(&self) {
        println!("| {:^5} | {:^10} | {:^10} |", "ID", "Holder", "Balance");
        println!(
            "| {:^5} | {:^10} | {:^10} |",
            "-----", "----------", "----------"
        );

        self.accounts.iter().for_each(|account| {
            println!(
                "| {:^5} | {:^10} | {:^10} |",
                account.id, account.holder, account.balance
            );
        });

        println!(
            "| {:^5} | {:^10} | {:^10} |",
            "-----", "----------", "----------"
        );

        println!("| {:^5} | {:^10} | {:^10} |", "", "Total", self.total_balance());

        println!(
            "| {:^5} | {:^10} | {:^10} |",
            "-----", "----------", "----------"
        );
    }

    fn summary_by(&self, account_id: u32) {
        println!("| {:^5} | {:^10} | {:^10} |", "ID", "Holder", "Balance");
        println!(
            "| {:^5} | {:^10} | {:^10} |",
            "-----", "----------", "----------"
        );

        for account in &self.accounts {
            if account.id == account_id {
                println!(
                    "| {:^5} | {:^10} | {:^10} |",
                    account.id, account.holder, account.balance
                );
            }
        }

        println!(
            "| {:^5} | {:^10} | {:^10} |",
            "-----", "----------", "----------"
        );
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }
}

fn main() {
    let mut bank = Bank::new();
    // let account = Account::new(1, "Alice".to_string());
    let account0 = Account::new(1, String::from("Alice"));
    let account1 = Account::new(2, String::from("Linda"));
    let account2 = Account::new(3, String::from("Bob"));
    let account3 = Account::new(4, String::from("Charlie"));

    bank.add_account(account0);
    bank.add_account(account1);
    bank.add_account(account2);
    bank.add_account(account3);

    bank.deposit(1, &100);
    bank.deposit(2, &200);
    bank.deposit(3, &300);
    bank.deposit(4, &400);

    bank.withdraw(1, 50);
    bank.withdraw(2, 100);
    bank.withdraw(3, 150);
    bank.withdraw(4, 200);

    // bank.summary_by(1);
    bank.summary();
}
