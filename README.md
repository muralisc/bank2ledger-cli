
# Bank2ledger

Convert bank csv files into [ledger-cli](https://ledger-cli.org/) format.

From [ledger-cli.org](https://ledger-cli.org/),

> Ledger is a powerful, double-entry accounting system that is accessed from the
UNIX command-line.

However to use it we need out transactions to be encoded in ledger format. 
This tool lets us do that for any bank / finance handlers (Insurance, Stock
Brokers) who can furnish a csv of transactions

### How does it work

For a transaction snippet from Monzo csv:
```
tx_0000AZPPhBZWW3Pq4dXGYW,04/09/2023,02:02:01,Direct Debit,Hyperoptic,,Bills,-26.50,GBP,-26.50,GBP,HYP000000630597,,,HYP000000630597,,-26.50,
```
to
```
2023-09-04 * "Hyperoptic"
        Assets:Bank:Monzo        -26.50 GBP
        Expenses:Utilities
```



## Charles Schwab

### How to Get CSV file ?

Schwab.com > History

### Command to run
```
cargo run -- \
  --config $PATH_TO_BANK2LEDGER_CLI/config/schwab.toml \
  --transactions-csv $PATH_TO_CSV/schwab_Transactions_20230409-103308.csv \
  > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_schwab.ledger
```

## HSBC

### How to get CSV file?

From web portal
1. [Click](assets/hsbc/1_click.png)
2. [Filter](assets/hsbc/2_Filter_for_dates.png)
3. [Download](assets/hsbc/3_download.png)


### Command to run
```
cargo run -- \
    --config $PATH_TO_BANK2LEDGER_CLI/config/hsbc.toml \
    --transactions-csv $PATH_TO_CSV/hsbc_TrasactionHistory_09_April_2023.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_hsbc.ledger
```



## Amex

### How to get CSV file?

Statements & Activity > Previous Billing Periods > window

### Command to run
```
cargo run -- \
    --config $PATH_TO_BANK2LEDGER_CLI/config/amex.toml \
    --transactions-csv $PATH_TO_CSV/amex-dec.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_amex.ledger
```

## Monzo

### Command to run
```
cargo run -- \
    --config $PATH_TO_BANK2LEDGER_CLI/config/monzo.toml \
    --transactions-csv $PATH_TO_CSV/Monzo\ Data\ Export\ -\ July.csv \
    > $PATH_TO_LEDGER_FILES/ledger/ledger_2023_03Mar_monzo.ledger
```


## Identify new Account mappings

grep for WARN in log file
```
grep "WARN" bank2ledger.debug.log
```
