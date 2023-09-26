
# Bank2ledger

## Charles Schwab

### Get CSV

Schwab > History > Export

### Command to run
```
cargo run -- \
  --config ~/src/bank2ledger-cli/config/schwab.toml \
  --transactions-csv ~/shared_folders/transfer_work/charlse_Transactions_20230409-103308.csv \
  > /home/murali/shared_folders/minimal/Pensieve/textfiles/ledger/ledger_2023_03Mar_schwab.ledger
```

## HSBC

### Command to run
```
cargo run -- \
    --config ~/src/bank2ledger-cli/config/hsbc.toml \
    --transactions-csv ~/shared_folders/transfer_work/hsbc_TrasactionHistory_09_April_2023.csv \
    > /home/murali/shared_folders/minimal/Pensieve/textfiles/ledger/ledger_2023_03Mar_hsbc.ledger
```
How to get TrasactionHistory file for HSBC ?
From web portal
1. [Click](assets/hsbc/1_click.png)
2. [Filter](assets/hsbc/2_Filter_for_dates.png)
3. [Download](assets/hsbc/3_download.png)



## Amex


How to get TrasactionHistory file for AMEX ?

- [Statements & Activity] > Previous Billing Periods > choose window (11 Jul to 10 Aug) > CSV (Check for all details)

### Command to run
```
cargo run -- \
    --config ~/src/bank2ledger-cli/config/amex.toml \
    --transactions-csv ~/Downloads/amex-dec.csv > ~/Downloads/ledger_2022_12Dec_amex.txt \
    > /home/murali/shared_folders/minimal/Pensieve/textfiles/ledger/ledger_2023_03Mar_amex.ledger
```

## Monzo

### Command to run
```
cargo run -- \
    --config ~/src/bank2ledger-cli/config/monzo.toml \
    --transactions-csv ~/Downloads/Monzo Data Export - July.csv \
    > /home/murali/shared_folders/minimal/Pensieve/textfiles/ledger/ledger_2023_03Mar_monzo.ledger
```


## Identify new Account mappings

grep for WARN in log file
```
grep "WARN" bank2ledger.debug.log
```
