use crate::bank_account::Transaction::{Deposit, Withdraw};

type Date = String;

pub struct DateService {}

impl DateService{
    pub fn new() -> DateService {
        DateService {}
    }

    pub fn current_date(&self) -> Date {
        return "2023-05-05".to_string();
    }
}

#[derive(Clone)]
pub enum Transaction{
    Deposit(f32, Date),
    Withdraw(f32, Date),
}

pub struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    pub(crate) fn new() -> Account {
        Account{ transactions: vec![]}
    }

    pub fn make_deposit(&mut self, amount: f32, date: Date) -> Result<(), &'static str> {
        if amount < 0.0 {
            return Err("The deposit amount should be more than 0")
        }
        self.transactions.push(Deposit(amount, date));
        return Ok(());
    }

    pub fn make_withdraw(&mut self, amount: f32, date: Date) -> Result<(), &'static str>{
        if self.balance() - amount < 0.0 {
            return Err("No overdraft used")
        }
        self.transactions.push(Withdraw(amount, date));
        return Ok(())
    }

    fn balance(&self) -> f32 {
        return self.transactions.iter()
            .map(|t| -> f32 {
                match t {
                    Deposit(val, _) =>  *val,
                    Withdraw(val, _) => -val,
                }
            }).sum();
    }

    pub fn print_balance(&self, printer: &dyn AccountPrinter) {
        printer.print_balance(self.balance())
    }

    pub fn print_statement(&self, printer: &dyn AccountPrinter) {
        printer.print_transaction_history(self.transactions.to_vec());
    }
}

pub struct SysOutPrinter {
}

pub trait AccountPrinter {
    fn print_balance(&self, balance: f32) -> ();
    fn print_transaction_history(&self, transactions: Vec<Transaction>) -> ();
}
impl AccountPrinter for SysOutPrinter {
    fn print_balance(&self, balance: f32) -> (){
        println!("{}", format!("current balance: {}", balance))
    }

    fn print_transaction_history(&self, transactions: Vec<Transaction>) -> () {
        let header = "Date | Amount";

        let rows: Vec<String> = transactions.iter()
            .map(format_transaction)
            .collect();

        print!("{}\n{}", header, rows.join("\n"))
    }
}

fn format_transaction(transaction: &Transaction) -> String {
    match transaction {
        Withdraw(amount, date) => format!(" {} | {}", date, -amount),
        Deposit(amount, date) => format!(" {} | {}", date, amount),
    }
}

mod tests {

    use super::*;

    #[test]
    fn should_get_empty_balance_when_account_is_created() {
        let account = Account::new();

        assert_eq!(account.balance(), 0.0)
    }

    #[test]
    fn should_make_deposite() {
        let mut account = Account::new();
        let date_service = DateService::new();
        let result = account.make_deposit(50.0, date_service.current_date());

        assert_eq!(result.is_ok(), true);
        assert_eq!(account.balance(), 50.0)
    }

    #[test]
    fn should_not_make_deposit_under_() {
        let mut account = Account::new();
        let date_service = DateService::new();
        let result = account.make_deposit(-1.0,date_service.current_date());

        assert_eq!(result.is_err(), true);
        assert_eq!(account.balance(), 0.0)
    }

    #[test]
    fn should_make_withdraw() {
        let mut account = Account::new();
        let date_service = DateService::new();
        account.make_deposit(50.0, date_service.current_date());
        let result = account.make_withdraw(5.0, date_service.current_date());

        assert_eq!(result.is_ok(), true);
        assert_eq!(account.balance(), 45.0)
    }

    #[test]
    fn should_not_make_withdraw_if_overdraft() {
        let mut account = Account::new();
        let date_service = DateService::new();
        account.make_deposit(3.0, date_service.current_date());
        let result = account.make_withdraw(5.0, date_service.current_date());

        assert_eq!(result.is_err(), true);
        assert_eq!(account.balance(), 3.0)
    }
}