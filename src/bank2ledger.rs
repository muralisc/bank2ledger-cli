use crate::ledger_record::LedgerRecord;
use crate::settings::Settings;
use chrono::NaiveDate;
use regex::RegexBuilder;

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
        return NaiveDate::parse_from_str(date_str, "%d/%m/%Y").unwrap();
    }

    fn get_payee(&self, record: &csv::StringRecord) -> String {
        return record[self.settings.ledger_record_to_row.payee]
            .trim()
            .to_string();
    }

    fn get_second_accoutn(&self, record: &csv::StringRecord) -> String {
        let second_account_hint = &record[self.settings.ledger_record_to_row.second_account_hint];
        let amount = &record[self.settings.ledger_record_to_row.first_amount];

        let mapping;
        if '-' == amount.chars().nth(0).unwrap() {
            mapping = &self.settings.payee_to_second_account.expense;
        } else {
            mapping = &self.settings.payee_to_second_account.income;
        }


        for item in mapping {
            let re = RegexBuilder::new(&format!(r"{}", item.key)).case_insensitive(true).build().unwrap();
            match re.find(second_account_hint) {
                Some(mat) => {
                    log::debug!("Match {:?}", mat);
                    return item.value.to_string();
                }
                None => log::debug!("None")
            }
        }
        return self.settings.default_second_account.to_string();
    }

    fn get_first_amount(&self, record: &csv::StringRecord) -> String {
        return record[self.settings.ledger_record_to_row.first_amount].to_string();
    }
    fn get_first_amount_currency(&self, record: &csv::StringRecord) -> String {
        return record[self.settings.ledger_record_to_row.first_amount_currency].to_string();
    }

    fn should_exclude(&self, record: &csv::StringRecord) -> bool {
        let mut should_exclude: bool = false;
        for exclude_condition in &self.settings.exclude_conditions {
            log::debug!("Excluding condition: {:?}", exclude_condition);
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
        let mut reader = csv::Reader::from_path(&self.transactions_file_path).unwrap();
        for record in reader.records() {
            let record = record.unwrap();
            log::debug!("record: {:?}!", record);

            log::debug!("Length or row {}", record.len());
            log::debug!(
                "Address {}",
                &record[self.settings.ledger_record_to_row.meta.address],
            );

            if self.should_exclude(&record) {
                log::info!("Excluding row: {:?}", record);
                continue;
            }

            let date = self.get_date(&record);
            let payee = self.get_payee(&record);
            let default_first_account = &self.settings.default_first_account;
            let first_amount = self.get_first_amount(&record);
            let first_amount_currency = self.get_first_amount_currency(&record);
            let second_account = self.get_second_accoutn(&record);

            let lr = LedgerRecord::new(
                date,
                payee,
                default_first_account.to_string(),
                first_amount,
                first_amount_currency,
                second_account,
            );
            lr.print();
        }
    }
}
