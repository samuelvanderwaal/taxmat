use std::path::PathBuf;
use structopt::StructOpt;

use crate::formats::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "taxmat", about = "Polkadot staking csv tax formatter.")]
pub struct Opt {
    /// input CSV file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,

    /// output CSV file name
    #[structopt(parse(from_os_str))]
    pub output: PathBuf,

    /// input CSV format
    #[structopt(short = "i", long, default_value = "subscan")]
    pub input_format: String,

    /// output CSV format
    #[structopt(short = "o", long, default_value = "bitcointax")]
    pub output_format: String,

    /// DOT or KSM coin
    #[structopt(short, long, default_value = "DOT")]
    pub coin: Coin,

    /// year to parse results from
    #[structopt(short, long)]
    pub year: Option<i32>,

    /// year's quarter to parse results
    #[structopt(short, long, default_value = "all")]
    pub quarter: Quarter,
}
