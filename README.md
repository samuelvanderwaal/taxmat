## Taxmat


Parse various Polkadot staking rewards input files and convert to various formats for tax purposes. Currently supported formats:

Inputs:

* Subscan
* Kraken

Outputs:

* Bitcoin.tax

Subscan files require specifying the coin type either DOT or KSM. Kraken files will read the asset type and convert all staking types to standard coin names: ETH2.S => ETH, ATOM.S => ATOM, DOT.S => DOT, etc. 


### Install

On Linux install from source with `cargo`:

```bash
git clone https://github.com/samuelvanderwaal/taxmat.git
cd taxmat
cargo install --path ./
```



### API

USAGE:
    taxmat [OPTIONS] <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --coin <coin>                      specify DOT or KSM coin for subscan files [default: DOT]
    -i, --input-format <input-format>      input CSV format [default: subscan]
    -o, --output-format <output-format>    output CSV format [default: bitcointax]
    -q, --quarter <quarter>                year's quarter to parse results [default: all]
    -y, --year <year>                      year to parse results from

ARGS:
    <input>     input CSV file
    <output>    output CSV file name   



Examples:

Kraken --> Bitcoin.tax

```bash
taxmat -i kraken -y 2021 -q q2 kraken_ledgers.csv kraken_bitcointax.csv
```

Subscan --> Bitcoin.tax

```bash
taxmat-y 2021 -q 3 subscan.csv  subscan_bitcointax.csv
```

