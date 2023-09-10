

trait Account {
    fn deposit(&mut self, amount: u64);
    fn withdraw(&mut self, amount: u64)-> Result<f64,String>;
    fn balance(&self) -> u64;

}

struct BankAccount{
    account_number: u64,
    balance: u64,
    holder_name: String,
}

impl Account for BankAccount {
    
    fn balance(&self) -> u64 {
        println!("{} account balance is {} , bank account id is {}",self.holder_name,self.balance,self.account_number);
        self.balance
    }

    fn deposit(&mut self, amount: u64) {
        
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) -> Result<f64,String> {
        if amount > self.balance {
            println!("Insufficient funds");
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(amount as f64)
    }
}


fn transfer_money<T:Account,U:Account>(
    from : &mut T,
    to : &mut U,
    amount : u64
){
    let res = from.withdraw(amount);
    
    match res {
        Ok(amount) =>
            println!("Withdrawn {} from {}",amount,from.balance()),
        Err(_) => 
            return,
    };
        
    
    
    to.deposit(amount);
    println!("Transfered {} from {} to {}",amount,from.balance(),to.balance());
}



fn main() {
    
    let mut account1 = BankAccount{
        account_number: 1,
        balance: 100,
        holder_name: "John".to_string(),
    };

    let mut account2 = BankAccount{
        account_number: 2,
        balance: 200,
        holder_name: "Jane".to_string(),
    };

    transfer_money(&mut account1,&mut account2,50);
    transfer_money(&mut account2,&mut account1,100);

}
