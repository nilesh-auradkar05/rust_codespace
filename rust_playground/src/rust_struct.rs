/*
    - Structs: A data structure that allows to group multiple fields together under one name.
        - Used to create custom data types.
        - Can be used to group data together.
        - Can be used to represent a complex data type.
*/

// Defining a struct
struct BankAccount{
    owner: String,
    balance: f32,
}

impl BankAccount{

    // Can have 1 mutable borrow or any number of immutable borrows.

    // Method to withdrawl of money from the account
    fn withdraw_money(&mut self, amount: f32){
        
        println!("Withdrawing {} from account ownned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // Method to deposit money into the account
    fn deposit_money(&mut self, amount: f32){
        println!("Depositing {} into account owned by {}", amount, self.owner);
        self.balance += amount;
    }

    // Method to get the balance of the account
    fn get_balance(&self){
        println!("The balance of account owned by {} is {:.2}", self.owner, self.balance);
    }
}

fn main(){
    
    let mut account: BankAccount = BankAccount{
        owner: String::from("John Doe"),
        balance: 100_000.0,
    };

    // Immutable borrow to check the balance
    account.get_balance();
    
    // Mutable borrow to withdraw money from the account
    account.withdraw_money(5000.0);

    // Immutable borrow to check the balance
    account.get_balance();

    // Mutable borrow to deposit money into the account
    account.deposit_money(200_000.0);

    // Immutable borrow to check the balance
    account.get_balance();
    
    
}