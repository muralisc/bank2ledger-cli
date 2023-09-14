mod bank2ledger;
mod ledger_record;
mod settings;

use clap::Parser;
use tracing::info;
use tracing_core::Level;
use tracing_subscriber;

use bank2ledger::Bank2Ledger;
use settings::Settings;
use std::fs::File;

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
    let debug_file_appender = File::create("bank2ledger.debug.log").unwrap();
    let (non_blocking_debug, _guard) = tracing_appender::non_blocking(debug_file_appender);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(non_blocking_debug)
        .init();

    let args = Args::parse();
    info!("Config path: {}!", args.config);
    let settings = match Settings::new(&args.config) {
        Ok(settings) => settings,
        Err(error) => {
            println!("Error opening settings file, Exiting. Error: {:?}", error);
            return;
        }
    };
    info!("Bank CSV path: {}!", args.transactions_csv);

    let bank2ledger = Bank2Ledger::new(settings, args.transactions_csv);
    bank2ledger.print();
}
