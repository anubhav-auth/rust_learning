fn main() {

    // value is owned by s1
    // let s1 = String::from("RUST");
    // let len = calculate_length(&s1); 
    // with this we borrow s1 hence the ownership remains with s1
    // println!("Length of '{}' is {}", s1, len);
    

    //2 can only be 1 owner at a time
    // let s2 = s1; // here ownership transferred to s2
    // let s2 = s1.clone(); // use clone to put value in s2 without changing ownership
    // println!("{}",s1);
    // println!("{}",s2);


    // let mut _s = 1;
    // let _r = & mut _s;

    // *_r+=3; // * make a reference to the value
    
    // println!("{}",_r);
     let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 56.52
     };

    account.check_balance();
    
    account.withdraw(24.5);
    account.check_balance();
}


// -3 when owner goes out of scope the calue will be dropped
// here s1 is not in scope so the value is dropped from memory

fn calculate_length(s: &String) -> usize{
    s.len()
}


struct BankAccount{
    owner: String,
    balance:f64
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64) {
        println!("Wothdrwing {} from account owner by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("the balance in the account is {}", self.balance);
    }
}