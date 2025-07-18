const INTEREST_RATE:f32 =0.05; //A 5% interest rate 

fn main(){
let account_balance:f32 = 1000.0;  // p = Account_Balance * INTEREST_RATE * time
let one_year: f32 = 365.0;

let account_balance: f32 =   account_balance * INTEREST_RATE / one_year;

print!("The acount of mony you will have is in the netx 12 moues is {}",  account_balance)
}
