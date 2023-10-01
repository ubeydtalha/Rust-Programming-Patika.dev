
use core::num;
use std::{fmt, error::Error};

#[derive(Debug)]
enum AccountErrors {
    InsufficientFunds(String),
    InvalidAmount(String),
}

#[derive(Debug)]
struct Account {
    name: String,
    balance: f32,
}

impl Account {

    fn deposit (&mut self,amount : f32) 
     -> Result<(),AccountErrors>
    {
        if amount > 0.0 {
            self.balance += amount;
            return Ok(());
        }
        Err(AccountErrors::InvalidAmount("Amount must be greater than 0".to_string()))
    }

    fn withdraw (&mut self,amount : f32)
     -> Result<(),AccountErrors>
     {
        if self.balance >= amount {
            self.balance -= amount;
            return Ok(());
        }
        Err(AccountErrors::InsufficientFunds("Insufficient funds".to_string()))
    }

}

#[derive(Debug)]

struct Bank {
    name: String,
    accounts: Vec<Account>,
}

impl Bank {

    fn new (name : String) -> Bank {
        Bank {
            name,
            accounts: Vec::new(),
        }
    }


}

trait Atm  {
    
    fn transfer_money(
        mut account_1 : Account,
        mut account_2 : Account,
        amount : f32,
    ) -> Result<((Account,Account)),AccountErrors>
    
    {
        let res_1 = account_1.withdraw(amount);
        match res_1 {
            Ok(()) => {
                let res_2 = account_2.deposit(amount);
                match res_2 {
                    Ok(()) => {
                        return Ok((account_1,account_2));
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            Err(e) => {
                return Err(e);
            }
        }
        
    }
}

impl Atm for Bank {}

fn main() {

    let my_bank = Bank::new("My Bank".to_string());
    
    let mut account_1 = Account {
        name: "John".to_string(),
        balance: 100.0,
    };

    let mut account_2 = Account {
        name: "Jane".to_string(),
        balance: 100.0,
    };

    println!("Account 1: {:?}",account_1);
    println!("Account 2: {:?}",account_2);

    let res = Bank::transfer_money(account_1,account_2,150.0);
    match res {
        Ok((account_1,account_2)) => {
            println!("Transfer successful");
            println!("Account 1: {:?}",account_1);
            println!("Account 2: {:?}",account_2);
        },
        Err(e) => {
            println!("Error: {:?}",e);
        }
    }


        
}
