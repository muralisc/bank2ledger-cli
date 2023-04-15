use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub description: usize,
    pub address: usize,
    pub transaction_type: usize,
}

#[derive(Debug, Deserialize)]
pub struct LedgerRecordToRow {
    pub date: usize,
    pub payee: usize,
    pub first_account: Option<usize>,
    pub second_account_hint: usize,
    pub first_amount: usize,
    pub first_amount_currency: Option<usize>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize)]
pub struct ExcludeCondition {
    pub column: usize,
    pub value: String,
    pub operation: String,
}

#[derive(Debug, Deserialize)]
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
