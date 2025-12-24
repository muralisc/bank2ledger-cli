use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::collections::HashMap;

// Configs used to map a csv to ledger records

// More like csv row to ledger record
// Setting to map a csv row to ledger record
#[derive(Debug, Deserialize)]
pub struct LedgerRecordToRow {
    // CSV column containing the date
    pub date: usize,
    // CSV column containing the payee
    pub payee: usize,
    pub second_account_hint: Vec<usize>,
    // CSV column containing the first amount
    // (if credit and debit are in multiple columns,
    // this has credit column)
    pub first_amount: usize,
    // Only used if there is a separate column for debit
    pub first_amount_debit: Option<usize>,
    // CSV column containing the first amount currency
    pub first_amount_currency: Option<usize>,
    pub comment: Option<Vec<usize>>,
    // CSV column used to find the first account
    // Usually this is not set and is taken from
    // default_first_account setting in global option
    pub first_account_hint: Option<Vec<usize>>
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum ExcludeCondition {
    ColumnContainsValue {
        column: usize,
        value: String,
        operation: String,
    },
    RecordLen(usize),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mapping {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SecondAccountHintMapping {
    pub expense: Vec<Mapping>,
    pub income: Vec<Mapping>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub default_first_account: String,
    pub default_second_account: String,
    pub csv_has_headers: bool,
    pub date_format: String,
    pub date_regex: String,
    pub first_amount_currency_default: String,
    pub ledger_record_to_row: LedgerRecordToRow,
    pub exclude_conditions: Vec<ExcludeCondition>,
    pub second_account_hint_mapping: SecondAccountHintMapping,
    pub minus_indicates_expense: Option<bool>,
    pub delimiter: Option<String>,
    pub first_account_hint_mapping: Option<Vec<Mapping>>,
}

impl Settings {
    pub fn new(config_file: &String) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(&config_file))
            .add_source(Environment::with_prefix("bank2ledger"))
            .set_override("database.url", "postgres://")?
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }

    pub fn get_delimiter(&self) -> u8 {
        let delimiter_map: HashMap<&str, u8> =
            HashMap::from([(",", b','), ("comma", b','), ("tab", b'\t')]);
        match &self.delimiter {
            None => delimiter_map["comma"],
            Some(delimiter_string) => delimiter_map[&delimiter_string as &str],
        }
    }
}
