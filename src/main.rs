use anyhow::Result;
use chrono::prelude::*;
use std::process;
use structopt::StructOpt;

use taxmat::opt::Opt;
use taxmat::formats::*;

fn main() -> Result<()> {
    let options = Opt::from_args();

    let input_format: InputFormat = match &options.input_format.to_lowercase()[..] {
        "subscan" => InputFormat::Subscan,
        "kraken" => InputFormat::Kraken,
        _ => {
            println!("Invalid input format!");
            process::exit(1);
        }
    };

    let output_format: OutputFormat = match &options.output_format.to_lowercase()[..] {
        "bitcointax" | "bitcoin.tax" => OutputFormat::BitcoinTax,
        _ => {
            println!("Invalid output format!");
            process::exit(1);
        }
    };

    match input_format {
        InputFormat::Subscan => parse_records::<Subscan>(&options, &output_format)?,
        InputFormat::Kraken => parse_kraken_file(&options, &output_format)?,
    }

    Ok(())
}

fn parse_records<D: InputRecord + serde::de::DeserializeOwned>(options: &Opt, output_format: &OutputFormat) -> Result<()> {
    let symbol = options.coin;

    let (start_date, end_date) = get_date_range(&options);

    let mut rdr = csv::Reader::from_path(&options.input)?;
    let mut wtr = csv::Writer::from_path(&options.output)?;
    
    for result in rdr.deserialize() {
        let res: D = result?;
    
        let date = NaiveDateTime::parse_from_str(&res.get_date()[..],"%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) {
            let record = match output_format {
                OutputFormat::BitcoinTax => OutputRecord::BT(BitcoinTax::create(date, res.get_amount(), symbol)),
            };

            wtr.serialize(record)?;
        }
    }

    Ok(())
}

fn parse_kraken_file(options: &Opt, output_format: &OutputFormat) -> Result<()> {
    let symbol = options.coin;

    let (start_date, end_date) = get_date_range(&options);

    let mut rdr = csv::Reader::from_path(&options.input)?;
    let mut wtr = csv::Writer::from_path(&options.output)?;

    for result in rdr.deserialize() {
        let res: Kraken = result?;

        let date = NaiveDateTime::parse_from_str(&res.get_date()[..],"%Y-%m-%d %H:%M:%S")?;

        if (start_date <= date) && (date <= end_date) {

            if res.action == "staking" {
                let record = match output_format {
                    OutputFormat::BitcoinTax => OutputRecord::BT(BitcoinTax::create(date, res.get_amount(), symbol)),
                };

                wtr.serialize(record)?;
            }
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
