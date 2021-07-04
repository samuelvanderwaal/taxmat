use anyhow::{bail, Error as AnyError, Result};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait InputRecord {
    fn get_date(&self) -> &String;

    fn get_amount(&self) -> f64;
}

#[derive(Debug, Deserialize)]
pub struct Subscan {
    #[serde(rename = "Event ID")]
    pub event_id: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Block")]
    pub block: u64,

    #[serde(rename = "Extrinsic Hash")]
    pub extrinsic: String,

    #[serde(rename = "Value")]
    pub amount: f64,

    #[serde(rename = "Action")]
    pub action: String,
}

impl InputRecord for Subscan {
    fn get_date(&self) -> &String {
        &self.date
    }

    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Deserialize)]
pub struct Kraken {
    pub txid: String,
    pub refid: String,

    #[serde(rename = "time")]
    pub date: String,

    #[serde(rename = "type")]
    pub action: String,

    pub aclass: String,
    pub asset: String,
    pub amount: f64,
    pub fee: f64,
}

impl InputRecord for Kraken {
    fn get_date(&self) -> &String {
        &self.date
    }

    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Deserialize)]
pub enum InputFormat {
    Subscan,
    Kraken,
}

#[derive(Debug)]
pub enum OutputFormat {
    BitcoinTax,
}

#[derive(Debug)]
pub enum Quarter {
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
            "q1" | "1" => Ok(Quarter::Q1),
            "q2" | "2" => Ok(Quarter::Q2),
            "q3" | "3" => Ok(Quarter::Q3),
            "q4" | "4" => Ok(Quarter::Q4),
            "all" => Ok(Quarter::ALL),
            _ => bail!("Invalid quarter!"),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum OutputRecord {
    BT(BitcoinTax),
}

#[derive(Debug, Serialize)]
pub struct BitcoinTax {
    date: NaiveDateTime,
    action: String,
    account: String,
    symbol: Coin,
    volume: f64,
}

impl BitcoinTax {
    pub fn create(date: NaiveDateTime, volume: f64, symbol: Coin) -> Self {
        Self {
            date,
            action: "INCOME".into(),
            account: "Polkadot Staking".into(),
            symbol,
            volume,
        }
    }
}

#[derive(Debug, Serialize)]
enum Currency {
    USD,
    GBP,
    EUR,
}

impl FromStr for Currency {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "usd" => Ok(Currency::USD),
            "gbp" => Ok(Currency::GBP),
            "eur" => Ok(Currency::EUR),
            _ => bail!("Invalid currency type!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Coin {
    DOT,
    KSM,
    ATOM,
    ETH,
    SOL,
    KAVA,
    ADA,
    XTZ,
}

impl FromStr for Coin {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "dot" | "dot.s" => Ok(Coin::DOT),
            "ksm" | "ksm.s" => Ok(Coin::KSM),
            "atom" | "atom.s" => Ok(Coin::ATOM),
            "eth" | "eth.s" | "eth2" | "eth2.s" => Ok(Coin::ETH),
            "sol" | "sol.s" => Ok(Coin::SOL),
            "kava" | "kava.s" => Ok(Coin::KAVA),
            "ada" | "ada.s" => Ok(Coin::ADA),
            "xtz" | "xtz.s" => Ok(Coin::XTZ),
            _ => bail!("Invalid coin type!"),
        }
    }
}
