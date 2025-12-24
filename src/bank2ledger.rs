use crate::ledger_record::LedgerRecord;
use crate::settings::{ExcludeCondition, Mapping, Settings};
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
        debug!("Processing date string: \"{}\" with regex {}", date_str, self.settings.date_regex);
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
        debug!("Checking in record is expense. Checking if there is a debit column");
        match &self.settings.ledger_record_to_row.first_amount_debit {
            Some(first_amount_debit_col) => {
                let debit_amount_string = record[*first_amount_debit_col].to_string();
                // If debit column exist and its not null then this record denotes an expense.
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

    fn get_comment(&self, record: &csv::StringRecord) -> Option<String> {
        let mut comment_str : String = Default::default();
        if let Some(comment_idxes) = &self.settings.ledger_record_to_row.comment {
            for idx in comment_idxes.iter() {
                comment_str.push_str(&record[*idx]);
                comment_str.push_str(" | ");
            }
            return Some(comment_str);
        }
        return None;
    }

    fn get_first_account(&self, record: &csv::StringRecord) -> String {
        if let Some(first_account_hint_cols) = &self.settings.ledger_record_to_row.first_account_hint {
            let first_account_hint: String = first_account_hint_cols
            .iter()
            .map(|i| record[*i].to_string())
            .collect::<Vec<String>>()
            .join(" ");
            if let Some(first_account_hint_mapping) = &self.settings.first_account_hint_mapping {
                for item in first_account_hint_mapping {
                    let re = RegexBuilder::new(&format!(r"{}", item.key))
                        .case_insensitive(true)
                        .build()
                        .unwrap();
                    match re.find(&first_account_hint) {
                        Some(mat) => {
                            debug!("Match for first account: {:?} hint: {:?}, value: {:?}", mat, item.key, item.value);
                            return item.value.to_string();
                        }
                        None => debug!(
                            "First account mapped to None for hint {:?} for regex {:?}",
                            first_account_hint, item.key
                        ),
                    }
                }
            } else {
                panic!("first_account_hint provided without first_account_hint_mapping: {:?}", first_account_hint)
            }
        }
        return self.settings.default_first_account.to_string();
    }
    fn get_second_account(&self, record: &csv::StringRecord) -> String {
        let second_account_hint: String = self
            .settings
            .ledger_record_to_row
            .second_account_hint
            .iter()
            .map(|i| record[*i].to_string())
            .collect::<Vec<String>>()
            .join(" ");
        debug!(
            "Getting second account with hint: {:?}",
            second_account_hint
        );
        let amount = &record[self.settings.ledger_record_to_row.first_amount];

        let mut mapping: Vec<Mapping> = self.settings.second_account_hint_mapping.expense.clone();
        if !self.is_record_expense(record) {
            // If the amount is an income it could be a refund.
            // So lets concat both the income and expense maps.
            let income_mapping = self.settings.second_account_hint_mapping.income.clone();
            mapping.extend(income_mapping);
            debug!("Income Mapping {:?}", mapping);
        }

        for item in mapping {
            let re = RegexBuilder::new(&format!(r"{}", item.key))
                .case_insensitive(true)
                .build()
                .unwrap();
            match re.find(&second_account_hint) {
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
    //                                  ^^^^^^^^^^ -------> first amount
    //  Expenses:UnaccountedExpenses
    fn get_first_amount(&self, record: &csv::StringRecord) -> String {
        return match self.settings.ledger_record_to_row.first_amount_debit {
            Some(first_amount_debit_col) => {
                if record[first_amount_debit_col].trim().is_empty() {
                    self.get_first_amount_single_field(
                        record,
                        self.settings.ledger_record_to_row.first_amount,
                        Some(true),
                    )
                } else {
                    self.get_first_amount_single_field(record, first_amount_debit_col, Some(false))
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

        let amount_string_dirty = record[amount_col].trim().to_string();
        let amount_string = amount_string_dirty.replace("$", "");
        // replace $ symbols if any !
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
            match exclude_condition {
                ExcludeCondition::ColumnContainsValue {
                    column,
                    value,
                    operation,
                } => {
                    let column_under_check = &record[*column];
                    if operation == "contains" {
                        if column_under_check.contains(&*value.as_str()) {
                            should_exclude = true;
                        }
                    } else if operation == "equal" {
                        if column_under_check == *value {
                            should_exclude = true;
                        }
                    }
                }
                ExcludeCondition::RecordLen(record_len) => {
                    if *record_len == record.len() {
                        should_exclude = true
                    }
                    debug!(
                        "Excluding condition: {:?}, record len : {}, should_exclude: {}",
                        record_len,
                        record.len(),
                        should_exclude
                    );
                }
            }
            if should_exclude == true  {
                break;
            }
        }
        return should_exclude;
    }

    pub fn print(&self) {
        let mut reader = match csv::ReaderBuilder::new()
            .has_headers(self.settings.csv_has_headers)
            .flexible(true)
            .delimiter(self.settings.get_delimiter())
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
                self.get_first_account(&record),
                self.get_first_amount(&record),
                self.get_first_amount_currency(&record),
                self.get_second_account(&record),
                self.get_comment(&record),
            );
            lr.print();
        }
    }
}
