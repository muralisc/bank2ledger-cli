use chrono::NaiveDate;

pub struct LedgerRecord {
    date: NaiveDate,
    payee: String,
    first_account: String,
    first_amount: String,
    first_amount_currency: String,
    second_account: String,
}

impl LedgerRecord {
    pub fn new(
        date: NaiveDate,
        payee: String,
        first_account: String,
        first_amount: String,
        first_amount_currency: String,
        second_account: String,
    ) -> Self {
        LedgerRecord {
            date: date,
            payee: payee,
            first_account: first_account,
            first_amount: first_amount,
            first_amount_currency: first_amount_currency,
            second_account: second_account,
        }
    }
    pub fn print(&self) {
        let tab_as_spaces = "        ";
        println!("{} * \"{}\"", self.date, self.payee);
        println!(
            "{}{}{}{} {}",
            tab_as_spaces,
            self.first_account,
            tab_as_spaces,
            self.first_amount,
            self.first_amount_currency
        );
        println!("{}{}", tab_as_spaces, self.second_account);
        println!();
    }
}
