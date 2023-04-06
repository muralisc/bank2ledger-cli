mod bank2ledger;
mod ledger_record;
mod settings;

use clap::Parser;
use log::debug;

use bank2ledger::Bank2Ledger;
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
    // Read
    // Apply filters
    // Print

    env_logger::init();
    let args = Args::parse();
    debug!("Config path: {}!", args.config);
    let settings = Settings::new(&args.config).unwrap();
    debug!("Bank CSV path: {}!", args.transactions_csv);

    let bank2ledger = Bank2Ledger::new(settings, args.transactions_csv);
    bank2ledger.print();
}
