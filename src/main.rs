use anyhow::{anyhow, Result};
use chrono::prelude::*;
use csv::Terminator;
use std::process;
use structopt::StructOpt;

use taxmat::formats::*;
use taxmat::opt::Opt;

fn main() -> Result<()> {
    let options = Opt::from_args();

    let input_format: InputFormat = match &options.input_format.to_lowercase()[..] {
        "subscan" => InputFormat::Subscan,
        "kraken" => InputFormat::Kraken,
        "staketax" => InputFormat::StakeTax,
        _ => {
            println!("Invalid input format!");
            process::exit(1);
        }
    };

    let output_format: OutputFormat = match &options.output_format.to_lowercase()[..] {
        "bitcointax" | "bitcoin.tax" => OutputFormat::BitcoinTax,
        "cointracking" | "coin tracking" => OutputFormat::CoinTracking,
        _ => {
            println!("Invalid output format!");
            process::exit(1);
        }
    };

    match input_format {
        InputFormat::Subscan => parse_records::<Subscan>(&options, &output_format)?,
        // Kraken files have multiple types of coins
        InputFormat::Kraken => parse_kraken_file(&options, &output_format)?,
        InputFormat::StakeTax => parse_staketax(&options, &output_format)?,
    }

    Ok(())
}

fn parse_records<D: InputRecord + serde::de::DeserializeOwned>(
    options: &Opt,
    output_format: &OutputFormat,
) -> Result<()> {
    let symbol = options.coin;

    println!("symbol: {symbol:?}");

    let (start_date, end_date) = get_date_range(options)?;

    let mut rdr = csv::Reader::from_path(&options.input)?;
    let mut wtr = csv::Writer::from_path(&options.output)?;

    match output_format {
        OutputFormat::BitcoinTax => {
            wtr.write_record(["Date", "Action", "Account", "Symbol", "Volume"])?;
        }
        OutputFormat::CoinTracking => {
            wtr.write_record([
                "Type",
                "Buy Amount",
                "Buy Currency",
                "Sell Amount",
                "Sell Currency",
                "Fee",
                "Fee Currency",
                "Exchange",
                "Trade-Group",
                "Comment",
                "Date",
                "Tx-ID",
                "Buy Value in Account Currency",
            ])?;
        }
    }

    for result in rdr.deserialize() {
        let res: D = result?;

        let date = NaiveDateTime::parse_from_str(&res.get_date()[..], "%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) {
            let record = match output_format {
                OutputFormat::BitcoinTax => {
                    OutputRecord::BT(BitcoinTax::create(date, res.get_amount(), symbol))
                }
                OutputFormat::CoinTracking => OutputRecord::CT(CoinTracking::create(
                    res.get_amount(),
                    symbol.into(),
                    "Polkadot Staking".into(),
                    "Self-Staking".to_string(),
                    date,
                )),
            };

            wtr.serialize(record)?;
        }
    }

    Ok(())
}

fn parse_staketax(options: &Opt, output_format: &OutputFormat) -> Result<()> {
    let symbol = options.coin;

    let (start_date, end_date) = get_date_range(options)?;

    let mut rdr = csv::Reader::from_path(&options.input)?;
    let mut wtr = csv::WriterBuilder::new()
        .terminator(Terminator::CRLF)
        .from_path(&options.output)?;

    for result in rdr.deserialize() {
        let res: StakeTax = result?;

        let date = NaiveDateTime::parse_from_str(&res.get_date()[..], "%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) && (res.tx_type == "STAKING") {
            let record = match output_format {
                OutputFormat::BitcoinTax => {
                    OutputRecord::BT(BitcoinTax::create(date, res.get_amount(), symbol))
                }
                _ => panic!("Not currently supported"),
            };

            wtr.serialize(record)?;
        }
    }

    Ok(())
}

fn parse_kraken_file(options: &Opt, output_format: &OutputFormat) -> Result<()> {
    // let symbol = options.coin;

    let (start_date, end_date) = get_date_range(options)?;

    let mut rdr = csv::Reader::from_path(&options.input)?;
    let mut wtr = csv::Writer::from_path(&options.output)?;

    for result in rdr.deserialize() {
        let res: Kraken = result?;

        let date = NaiveDateTime::parse_from_str(&res.get_date()[..], "%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) && res.action == "staking" {
            let coin_opt = res.asset.parse::<Coin>();

            let coin = match coin_opt {
                Ok(coin) => coin,
                Err(e) => {
                    println!("Invalid coin: {}", res.asset);
                    panic!("{}", e);
                }
            };
            let record = match output_format {
                OutputFormat::BitcoinTax => {
                    OutputRecord::BT(BitcoinTax::create(date, res.get_amount(), coin))
                }
                _ => panic!("Not currently supported"),
            };

            wtr.serialize(record)?;
        }
    }

    Ok(())
}

fn get_date_range(options: &Opt) -> Result<(NaiveDateTime, NaiveDateTime)> {
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

    let start_date = NaiveDate::from_ymd_opt(year, start_month, start_day)
        .ok_or(anyhow!("failed to create ymd"))?
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow!("failed to create hms"))?;
    let end_date = NaiveDate::from_ymd_opt(year, end_month, end_day)
        .ok_or(anyhow!("failed to create ymd"))?
        .and_hms_opt(23, 59, 59)
        .ok_or(anyhow!("failed to create hms"))?;

    Ok((start_date, end_date))
}
