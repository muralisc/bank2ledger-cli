use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

// Configs used to map a csv to ledger records

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub description: usize,
    pub address: Option<usize>,
    pub transaction_type: usize,
}

// More like csv row to ledger record
// Setting to map a csv row to ledger record
#[derive(Debug, Deserialize)]
pub struct LedgerRecordToRow {
    // CSV column containing the date
    pub date: usize,
    // CSV column containing the payee
    pub payee: usize,
    // CSV column used to find the first account
    // Usually this is not set and is taken from
    // default_first_account setting in global option
    pub first_account: Option<usize>,
    pub second_account_hint: usize,
    // CSV column containing the first amount
    pub first_amount: usize,
    // CSV column containing the first amount currency
    pub first_amount_currency: Option<usize>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct ExcludeCondition {
    pub column: usize,
    pub value: String,
    pub operation: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Mapping {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct PayeeSecondAccountMapping {
    pub expense: Vec<Mapping>,
    pub income: Vec<Mapping>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub default_first_account: String,
    pub default_second_account: String,
    pub csv_has_headers: bool,
    pub date_format: String,
    pub date_regex: String,
    pub ledger_record_to_row: LedgerRecordToRow,
    pub exclude_conditions: Vec<ExcludeCondition>,
    pub payee_to_second_account: PayeeSecondAccountMapping,
    pub minus_indicates_expense: Option<bool>,
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
}
