use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LedgerRecordToRow {
    pub date: usize,
    pub payee: usize,
    pub first_account: Option<usize>,
    pub second_account_hint: usize,
    pub first_amount: usize,
    pub first_amount_currency: usize,
    pub description: usize,
    pub address: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub default_first_account: String,
    pub ledger_record_to_row: LedgerRecordToRow,
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
