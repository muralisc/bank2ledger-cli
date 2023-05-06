


cargo run -- \
  --config ~/src/bank2ledger-cli/config/schwab.toml \
  --transactions-csv ~/shared_folders/transfer_work/charlse_Transactions_20230409-103308.csv > /home/murali/shared_folders/minimal/Pensieve/textfiles/ledger/ledger_2023_03Mar_schwab.ledger

cargo run -- --config config/hsbc.toml --transactions-csv ~/shared_folders/transfer_work/hsbc_TrasactionHistory_09_April_2023.csv
