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
        let date = &record[settings.as_ref().unwrap().ledger_record_to_row.date].trim();
        let payee = &record[settings.as_ref().unwrap().ledger_record_to_row.payee].trim();
        
        info!("Length or row {}", record.len());
        println!( "\n{date} * \"{payee}\"");
        println!(
            "{tab_as_spaces} {default_first_account} {tab_as_spaces} {first_amount} {first_amount_currency}",
            default_first_account = settings.as_ref().unwrap().default_first_account,
            first_amount = &record[settings.as_ref().unwrap().ledger_record_to_row.first_amount],
            first_amount_currency = &record[settings
                .as_ref()
                .unwrap()
                .ledger_record_to_row
                .first_amount_currency],
        );
        debug!(
            "Payee {}",
            &record[settings.as_ref().unwrap().ledger_record_to_row.payee],
        );
        debug!(
            "Address {}",
            &record[settings.as_ref().unwrap().ledger_record_to_row.address],
        );
        println!(
            "{tab_as_spaces} {second_account_hint}",
            second_account_hint = &record[settings
                .as_ref()
                .unwrap()
                .ledger_record_to_row
                .second_account_hint],
        );
    }
}
