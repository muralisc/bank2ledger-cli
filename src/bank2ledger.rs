use crate::ledger_record::LedgerRecord;
use crate::settings::{Mapping, Settings};
use chrono::NaiveDate;
use regex::RegexBuilder;
use tracing::{debug, info, warn};

pub struct Bank2Ledger {
    settings: Settings,
    transactions_file_path: String,
}

impl Bank2Ledger {
    pub fn new(settings: Settings, filepath: String) -> Self {
        Bank2Ledger {
            settings: settings,
            transactions_file_path: filepath,
        }
    }

    fn get_date(&self, record: &csv::StringRecord) -> NaiveDate {
        let date_str = &record[self.settings.ledger_record_to_row.date].trim();
        debug!("Processing date string: \"{}\" of record", date_str);
        let re = RegexBuilder::new(&format!(r"{}", self.settings.date_regex))
            .build()
            .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("Date matched with regex: {}", cleaned_date.as_str());
        return NaiveDate::parse_from_str(cleaned_date.as_str(), &self.settings.date_format)
            .unwrap();
    }

    fn get_payee(&self, record: &csv::StringRecord) -> String {
        return record[self.settings.ledger_record_to_row.payee]
            .trim()
            .to_string();
    }

    fn is_record_expense(&self, record: &csv::StringRecord) -> bool {
        match &self.settings.ledger_record_to_row.first_amount_debit {
            Some(first_amount_debit_col) => {
                let debit_amount_string = record[*first_amount_debit_col].to_string();
                return !debit_amount_string.trim().is_empty();
            }
            None => {
                let amount = &record[self.settings.ledger_record_to_row.first_amount];
                return self.is_amount_expense(amount);
            }
        }
    }

    fn is_amount_expense(&self, amount: &str) -> bool {
        let sign_present = '-' == amount.chars().nth(0).unwrap();
        match &self.settings.minus_indicates_expense {
            Some(minus_indicates_expense_value) => {
                if *minus_indicates_expense_value {
                    return sign_present;
                } else {
                    return !sign_present;
                }
            }
            None => return sign_present,
        }
    }

    fn get_second_account(&self, record: &csv::StringRecord) -> String {
        let second_account_hint = &record[self.settings.ledger_record_to_row.second_account_hint];
        debug!(
            "Getting second account with hint: {:?}",
            second_account_hint
        );
        let amount = &record[self.settings.ledger_record_to_row.first_amount];

        let mut mapping: Vec<Mapping> = self.settings.payee_to_second_account.expense.clone();
        if !self.is_record_expense(record) {
            // If the amount is an income it could be a refund.
            // So lets concat both the income and expense maps.
            let income_mapping = self.settings.payee_to_second_account.income.clone();
            mapping.extend(income_mapping);
            debug!("Income Mapping {:?}", mapping);
        }

        for item in mapping {
            let re = RegexBuilder::new(&format!(r"{}", item.key))
                .case_insensitive(true)
                .build()
                .unwrap();
            match re.find(second_account_hint) {
                Some(mat) => {
                    debug!("Match for second account {:?}", mat);
                    return item.value.to_string();
                }
                None => debug!(
                    "Second account mapped to None for hint {:?} for regex {:?}",
                    second_account_hint, item.key
                ),
            }
        }
        warn!(
            "Second account mapped to Default {} for hint {:?} and amount type \"{}\", and amount {}",
            self.settings.default_second_account.to_string(),second_account_hint,
            if self.is_record_expense(record) {
                "Expense"
            } else {
                "Income"
            },
            amount
        );
        return self.settings.default_second_account.to_string();
    }

    // First Amount is the amount used for a fist Account
    // e.g:
    // 2023-07-29 * "Crown Cafe Bar"
    //  Assets:Bank:Monzo               -13.30 GBP
    //                                  ^^^^^^^^^^ -------> first account
    //  Expenses:UnaccountedExpenses
    fn get_first_amount(&self, record: &csv::StringRecord) -> String {
        return match &self.settings.ledger_record_to_row.first_amount_debit {
            Some(first_amount_debit_col) => {
                let debit_amount_string = record[*first_amount_debit_col].to_string();
                if debit_amount_string.trim().is_empty() {
                    self.get_first_amount_single_field(
                        record,
                        self.settings.ledger_record_to_row.first_amount,
                        Some(true),
                    )
                } else {
                    self.get_first_amount_single_field(record, *first_amount_debit_col, Some(false))
                }
            }
            // CSV has only a single column with both Credit and Debit
            None => self.get_first_amount_single_field(
                record,
                self.settings.ledger_record_to_row.first_amount,
                self.settings.minus_indicates_expense,
            ),
        };
    }

    fn get_first_amount_single_field(
        &self,
        record: &csv::StringRecord,
        amount_col: usize,
        minus_indicates_expense: Option<bool>,
    ) -> String {
        // Usually -(minus) indicates money leaving accout
        // But some banks have 'minum' in amout which indicates
        // income, in those cases
        // If minus_indicates_expense in csv we need to filp the sign

        let amount_string = record[amount_col].to_string();
        debug!("Checking first amount: {}", amount_string);
        match minus_indicates_expense {
            Some(minus_indicates_expense_value) => {
                debug!(
                    "minus_indicates_expense_value: {:?}",
                    minus_indicates_expense_value
                );
                if minus_indicates_expense_value == false {
                    // Flip
                    if amount_string.chars().nth(0).unwrap() == '-' {
                        return amount_string[1..].to_string();
                    }
                    return format!("-{}", amount_string);
                }
            }
            None => (),
        }
        return amount_string;
    }

    fn get_first_amount_currency(&self, record: &csv::StringRecord) -> String {
        match self.settings.ledger_record_to_row.first_amount_currency {
            None => return self.settings.first_amount_currency_default.to_string(),
            Some(first_amount_currency) => {
                return record[first_amount_currency].to_string();
            }
        }
    }

    fn should_exclude(&self, record: &csv::StringRecord) -> bool {
        let mut should_exclude: bool = false;
        for exclude_condition in &self.settings.exclude_conditions {
            debug!("Checking Excluding condition: {:?}", exclude_condition);
            let column_under_check = &record[exclude_condition.column];
            if exclude_condition.operation == "contains" {
                if column_under_check.contains(&exclude_condition.value) {
                    should_exclude = true;
                }
            } else if exclude_condition.operation == "equal" {
                if column_under_check == &exclude_condition.value {
                    should_exclude = true;
                }
            }
        }
        return should_exclude;
    }

    pub fn print(&self) {
        let mut reader = match csv::ReaderBuilder::new()
            .has_headers(self.settings.csv_has_headers)
            .flexible(true)
            .delimiter(b',') // TODO use delimiter_text here
            .from_path(&self.transactions_file_path)
        {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        for record in reader.records() {
            let record = record.unwrap();
            debug!("<========== Starting processsing of new record/row ============>");
            debug!("Record : {:?}!", record);
            debug!("Length of Row/Record {}", record.len());

            if self.should_exclude(&record) {
                info!("Excluding row: {:?}", record);
                continue;
            }

            let lr = LedgerRecord::new(
                self.get_date(&record),
                self.get_payee(&record),
                self.settings.default_first_account.to_string(),
                self.get_first_amount(&record),
                self.get_first_amount_currency(&record),
                self.get_second_account(&record),
            );
            lr.print();
        }
    }
}
