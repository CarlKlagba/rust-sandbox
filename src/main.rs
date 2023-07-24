use crate::bank_account::{Account, DateService};
use crate::bank_account::SysOutPrinter;


mod fizzbuzz;
mod bank_account;
fn main() {
    let mut account = Account::new();
    let printer = SysOutPrinter{};

    let date_service = DateService::new();

    account.make_deposit(2034.65, date_service.current_date()).unwrap();
    account.make_deposit(99.65, date_service.current_date()).unwrap();
    account.make_withdraw(10.5, date_service.current_date()).unwrap();
    account.make_deposit(102.00, date_service.current_date()).unwrap();
    account.make_withdraw(23.4, date_service.current_date()).unwrap();

    account.print_balance(&printer);
    account.print_statement(&printer);


}