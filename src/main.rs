use anyhow::{bail, Error as AnyError, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "taxmat", about = "Polkadot staking csv tax formatter.")]
struct Opt {
    /// input CSV file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// output CSV file name
    #[structopt(parse(from_os_str))]
    output: PathBuf,

    /// output CSV format
    #[structopt(short, long, default_value = "bitcointax")]
    format: String,

    /// DOT or KSM coin
    #[structopt(short, long, default_value = "DOT")]
    coin: Coin,

    /// year to parse results from
    #[structopt(short, long)]
    year: Option<i32>,

    /// year's quarter to parse results
    #[structopt(short, long, default_value = "all")]
    quarter: Quarter,
}

#[derive(Debug)]
enum Format {
    BitcoinTax,
}

#[derive(Debug)]
enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
    ALL,
}

impl FromStr for Quarter {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "q1" => Ok(Quarter::Q1),
            "q2" => Ok(Quarter::Q2),
            "q3" => Ok(Quarter::Q3),
            "q4" => Ok(Quarter::Q4),
            "all" => Ok(Quarter::ALL),
            _ => bail!("Invalid quarter!"),
        }
    }
}

#[derive(Debug, Serialize)]
enum Currency {
    USD,
    GBP,
    EUR,
}

#[derive(Debug, Clone, Copy, Serialize)]
enum Coin {
    DOT,
    KSM,
}

impl FromStr for Coin {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "dot" => Ok(Coin::DOT),
            "ksm" => Ok(Coin::KSM),
            _ => bail!("Invalid coin type!"),
        }
    }
}


#[derive(Debug, Deserialize)]
struct Subscan {
    #[serde(rename = "Event ID")]
    event_id: String,

    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Block")]
    block: u64,

    #[serde(rename = "Extrinsic Hash")]
    extrinsic: String,

    #[serde(rename = "Value")]
    value: f64,

    #[serde(rename = "Action")]
    action: String,
}
#[derive(Debug, Serialize)]
enum OutputRecord {
    BT(BitcoinTax),
}

#[derive(Debug, Serialize)]
struct BitcoinTax {
    date: NaiveDateTime,
    action: String,
    account: String,
    symbol: Coin,
    volume: f64,
}

impl BitcoinTax {
    fn create(date: NaiveDateTime, volume: f64, symbol: Coin) -> Self {
        Self {
            date,
            action: "INCOME".into(),
            account: "Polkadot Staking".into(),
            symbol,
            volume,
        }
    }
}

fn main() -> Result<()> {
    let options = Opt::from_args();

    let format: Format = match &options.format.to_lowercase()[..] {
        "bitcointax" | "bitcoin.tax" => Format::BitcoinTax,
        _ => {
            println!("Invalid format!");
            process::exit(1);
        }
    };

    let symbol = options.coin;

    let (start_date, end_date) = get_date_range(&options);

    let mut rdr = csv::Reader::from_path(options.input)?;
    let mut wtr = csv::Writer::from_path(options.output)?;

    for result in rdr.deserialize() {
        let subscan: Subscan = result?;
        let date = NaiveDateTime::parse_from_str(&subscan.date[..],"%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) {
            let record = match format {
                Format::BitcoinTax => OutputRecord::BT(BitcoinTax::create(date, subscan.value, symbol)),
            };

            wtr.serialize(record)?;
        }
    }

    Ok(())
}

fn get_date_range(options: &Opt) -> (NaiveDateTime, NaiveDateTime) {
    let year = match options.year {
        Some(y) => y,
        None => Utc::now().naive_utc().year(),
    };
    
    let (start_month, start_day) = match options.quarter {
        Quarter::Q1 => (1, 1),
        Quarter::Q2 => (4, 1),
        Quarter::Q3 => (7, 1),
        Quarter::Q4 => (10, 1),
        Quarter::ALL => (1, 1),
    };
    
    let (end_month, end_day) = match options.quarter {
        Quarter::Q1 => (3, 31),
        Quarter::Q2 => (6, 30),
        Quarter::Q3 => (9, 30),
        Quarter::Q4 => (12, 31),
        Quarter::ALL => (12, 31),
    };
    
    let start_date = NaiveDate::from_ymd(year, start_month, start_day)
        .and_hms(0, 0, 0);
    let end_date = NaiveDate::from_ymd(year, end_month, end_day)
        .and_hms(23, 59, 59);
    (start_date, end_date)
}
