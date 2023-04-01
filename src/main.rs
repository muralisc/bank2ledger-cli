use chrono::NaiveDate;
use clap::Parser;
mod settings;
use log::{debug, info};

use settings::Settings;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Configuration for the bank / financial institution we are importing
    #[arg(short, long)]
    config: String,

    // CSV with transactions
    #[arg(short, long)]
    transactions_csv: String,
}

fn main() {
    let tab_as_spaces = "        ";
    env_logger::init();
    let args = Args::parse();
    debug!("Config path: {}!", args.config);
    let settings = Settings::new(&args.config);
    debug!("Bank CSV path: {}!", args.transactions_csv);
    let mut reader = csv::Reader::from_path(args.transactions_csv).unwrap();
    for record in reader.records() {
        let record = record.unwrap();

        debug!("record: {:?}!", record);
        let date_str = &record[settings.as_ref().unwrap().ledger_record_to_row.date].trim();
        let date = NaiveDate::parse_from_str(date_str, "%d/%m/%Y").unwrap();
        let payee = &record[settings.as_ref().unwrap().ledger_record_to_row.payee].trim();
        let second_account_hint = &record[settings
            .as_ref()
            .unwrap()
            .ledger_record_to_row
            .second_account_hint];
        let first_amount = &record[settings.as_ref().unwrap().ledger_record_to_row.first_amount];
        let first_amount_currency = &record[settings
            .as_ref()
            .unwrap()
            .ledger_record_to_row
            .first_amount_currency];

        let mut should_exclude: bool = false;
        for exclude_condition in &settings.as_ref().unwrap().exclude_conditions {
            debug!("Excluding condition: {:?}", exclude_condition);
            let column_under_check = &record[exclude_condition.column];
            if exclude_condition.operation == "contains" {
                if column_under_check.contains(&exclude_condition.value) {
                    should_exclude = true;
                }
            }
        }

        if should_exclude {
            info!("Excluding row: {:?}", record);
            continue;
        }

        debug!("Length or row {}", record.len());
        debug!("Payee {payee}");
        debug!(
            "Address {}",
            &record[settings.as_ref().unwrap().ledger_record_to_row.meta.address],
        );

        println!("\n{date} * \"{payee}\"");
        println!(
            "{tab_as_spaces} {default_first_account} {tab_as_spaces} {first_amount} {first_amount_currency}",
            default_first_account = settings.as_ref().unwrap().default_first_account,
        );

        println!("{tab_as_spaces} {second_account_hint}");
    }
}
