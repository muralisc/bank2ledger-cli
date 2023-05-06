use crate::ledger_record::LedgerRecord;
use crate::settings::Settings;
use chrono::NaiveDate;
use regex::RegexBuilder;
use tracing::{debug, info};

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
        debug!("date string : {}", date_str);
        let re = RegexBuilder::new(&format!(r"{}", self.settings.date_regex))
            .build()
            .unwrap();
        let cleaned_date = re.find(date_str).unwrap();
        debug!("date match: {}", cleaned_date.as_str());
        return NaiveDate::parse_from_str(cleaned_date.as_str(), &self.settings.date_format)
            .unwrap();
    }

    fn get_payee(&self, record: &csv::StringRecord) -> String {
        return record[self.settings.ledger_record_to_row.payee]
            .trim()
            .to_string();
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

    fn get_second_accoutn(&self, record: &csv::StringRecord) -> String {
        let second_account_hint = &record[self.settings.ledger_record_to_row.second_account_hint];
        debug!(
            "getting second account with hint: {:?}",
            second_account_hint
        );
        let amount = &record[self.settings.ledger_record_to_row.first_amount];

        let mapping;
        if self.is_amount_expense(amount) {
            mapping = &self.settings.payee_to_second_account.expense;
        } else {
            mapping = &self.settings.payee_to_second_account.income;
        }

        for item in mapping {
            let re = RegexBuilder::new(&format!(r"{}", item.key))
                .case_insensitive(true)
                .build()
                .unwrap();
            match re.find(second_account_hint) {
                Some(mat) => {
                    debug!("Match {:?}", mat);
                    return item.value.to_string();
                }
                None => debug!("Second account mapped to None"),
            }
        }
        return self.settings.default_second_account.to_string();
    }

    fn get_first_amount(&self, record: &csv::StringRecord) -> String {
        // If minus_indicates_expense in csv we need to filp the sign

        let amount_string = record[self.settings.ledger_record_to_row.first_amount].to_string();
        debug!("Checking amount: {}", amount_string);
        match &self.settings.minus_indicates_expense {
            Some(minus_indicates_expense_value) => {
                debug!(
                    "minus_indicates_expense_value: {:?}",
                    *minus_indicates_expense_value
                );
                if *minus_indicates_expense_value == false {
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
            None => return "GBP".to_string(),
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
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_path(&self.transactions_file_path)
            .unwrap();
        for record in reader.records() {
            let record = record.unwrap();
            debug!("record: {:?}!", record);
            debug!("Length or row {}", record.len());

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
                self.get_second_accoutn(&record),
            );
            lr.print();
        }
    }
}
